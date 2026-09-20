use patterns::{because, source};

pub struct TrainingMath;
source!(
    TrainingMath,
    "the loop trainer's step as the processor works it: softmax cross entropy over the actions, the rectified hidden units, a pointer's softmax over the slots a token may be copied from (Vinyals, Fortunato and Jaitly), and AdaGrad (Duchi, Hazan and Singer) moving every number once by the sum of its rows' gradients"
);

const TRAIN_BODY: &str = r#"
__device__ static inline int thread_of() {
    return blockIdx.x * blockDim.x + threadIdx.x;
}

__device__ static inline float rectified(float z) {
    return z > NONE ? z : NONE;
}

__device__ static inline void adagrad(float *w, float *sq, float g, float rate, float least) {
    *sq += g * g;
    *w -= rate * g / (sqrt(*sq) + least);
}

extern "C" __global__ void embedded(float *e_all, const float *table, const int *ids, const int *slot_ids, const int *row_slots, const int *places, const int *filled, const int *order, int offset, int n, int reach, int slots, int item) {
    int t = thread_of();
    if (t >= n * reach) return;
    int b = t / reach, s = t % reach, r = order[offset + b];
    float *e = e_all + (b * slots + s) * item;
    for (int i = NONE; i < item; i++) e[i] = NONE;
    if (s >= filled[r]) return;
    int q = row_slots[r] + filled[r] - ONE - s;
    for (int k = slot_ids[q]; k < slot_ids[q + ONE]; k++) {
        const float *v = table + ids[k] * item;
        for (int i = NONE; i < item; i++) e[i] += v[i];
    }
    int p = places[s];
    if (p >= NONE) {
        const float *v = table + p * item;
        for (int i = NONE; i < item; i++) e[i] += v[i];
    }
}



extern "C" __global__ void product(float *c, const float *a, const float *b, int m, int n, int inner, int lda, int ldb, int ldc, int a_turned, int b_turned) {
    __shared__ float sa[TILE][SIDE + ONE];
    __shared__ float sb[TILE][SIDE + ONE];
    int within = threadIdx.x;
    int tiles_n = (n + SIDE - ONE) / SIDE;
    int block = blockIdx.x;
    int row_at = (block / tiles_n) * SIDE, col_at = (block % tiles_n) * SIDE;
    int tr = within / TILE, tc = within % TILE;
    if (row_at >= m) return;
    float sum[PER][PER];
    for (int i = NONE; i < PER; i++) for (int j = NONE; j < PER; j++) sum[i][j] = NONE;
    for (int k_at = NONE; k_at < inner; k_at += TILE) {
        for (int part = NONE; part < PER; part++) {
            int load = within + part * TILE * TILE;
            int lr = load / TILE, lk = load % TILE;
            int ar = row_at + lr, ak = k_at + lk;
            sa[lk][lr] = (ar < m && ak < inner) ? (a_turned ? a[ak * lda + ar] : a[ar * lda + ak]) : NONE;
            int bk = k_at + (load / SIDE), bc = col_at + (load % SIDE);
            sb[load / SIDE][load % SIDE] = (bk < inner && bc < n) ? (b_turned ? b[bc * ldb + bk] : b[bk * ldb + bc]) : NONE;
        }
        __syncthreads();
        for (int q = NONE; q < TILE; q++) {
            float av[PER], bv[PER];
            for (int i = NONE; i < PER; i++) av[i] = sa[q][tr + i * TILE];
            for (int j = NONE; j < PER; j++) bv[j] = sb[q][tc + j * TILE];
            for (int i = NONE; i < PER; i++) for (int j = NONE; j < PER; j++) sum[i][j] += av[i] * bv[j];
        }
        __syncthreads();
    }
    for (int i = NONE; i < PER; i++) {
        for (int j = NONE; j < PER; j++) {
            int row = row_at + tr + i * TILE, col = col_at + tc + j * TILE;
            if (row < m && col < n) c[row * ldc + col] = sum[i][j];
        }
    }
}

extern "C" __global__ void hidden_rest(float *pre, const float *fill, const float *bias, const int *filled, const int *order, int offset, int n, int h) {
    int t = thread_of();
    if (t >= n * h) return;
    int b = t / h, j = t % h, r = order[offset + b];
    float z = pre[t] + bias[j];
    for (int s = NONE; s < filled[r]; s++) z += fill[s * h + j];
    pre[t] = z;
}

extern "C" __global__ void rectified_rows(float *out, const float *pre, int count) {
    int t = thread_of();
    if (t >= count) return;
    out[t] = rectified(pre[t]);
}

extern "C" __global__ void embed_point(float *d_e, const float *g, const float *pointed_part, const int *filled, const int *order, int offset, int n, int reach, int slots, int item) {
    int t = thread_of();
    if (t >= n * reach * item) return;
    int i = t % item, bs = t / item, b = bs / reach, s = bs % reach, r = order[offset + b];
    if (s >= filled[r]) return;
    float gs = g[b * slots + s];
    if (gs != NONE) d_e[(b * slots + s) * item + i] += gs * pointed_part[b * item + i];
}

extern "C" __global__ void slot_apply(float *weights, float *weights_sq, const float *grad, int count, float rate, float least) {
    int t = thread_of();
    if (t >= count) return;
    adagrad(weights + t, weights_sq + t, grad[t], rate, least);
}

extern "C" __global__ void outputs(float *d_out, float *loss, int *right, const float *out_bias, const int *wanted, const int *order, int offset, int n, int c_n, float smallest) {
    __shared__ float shared[TILE * TILE];
    __shared__ int shared_at[TILE * TILE];
    int b = blockIdx.x, t = threadIdx.x, width = blockDim.x;
    if (b >= n) return;
    int r = order[offset + b], want = wanted[r];
    float *d = d_out + b * c_n;
    float top = -(ONE / smallest);
    int best = NONE;
    for (int c = t; c < c_n; c += width) {
        d[c] += out_bias[c];
        if (d[c] >= top) {
            top = d[c];
            best = c;
        }
    }
    shared[t] = top;
    shared_at[t] = best;
    __syncthreads();
    for (int s = width / (ONE + ONE); s > NONE; s /= (ONE + ONE)) {
        if (t < s && (shared[t + s] > shared[t] || (shared[t + s] == shared[t] && shared_at[t + s] > shared_at[t]))) {
            shared[t] = shared[t + s];
            shared_at[t] = shared_at[t + s];
        }
        __syncthreads();
    }
    top = shared[NONE];
    best = shared_at[NONE];
    __syncthreads();
    float part = NONE;
    for (int c = t; c < c_n; c += width) part += exp(d[c] - top);
    shared[t] = part;
    __syncthreads();
    for (int s = width / (ONE + ONE); s > NONE; s /= (ONE + ONE)) {
        if (t < s) shared[t] += shared[t + s];
        __syncthreads();
    }
    float total = shared[NONE];
    for (int c = t; c < c_n; c += width) {
        float p = exp(d[c] - top) / total;
        if (c == want) loss[b] = -log(p > smallest ? p : smallest);
        d[c] = p - (c == want ? ONE : NONE);
    }
    if (t == NONE) right[offset + b] = best == want;
}

__device__ static inline int is_pointed(const int *pointed, int from, int to, int s) {
    for (int k = from; k < to; k++) {
        if (pointed[k] == s) return ONE;
    }
    return NONE;
}

extern "C" __global__ void point_scores(float *g, const float *pointed_part, const float *e_all, const int *pointable, const int *pointable_at, const int *pointed_at, const int *filled, const int *order, int offset, int n, int slots, int item) {
    int t = thread_of();
    if (t >= n * slots) return;
    int b = t / slots, s = t - b * slots;
    int r = order[offset + b];
    g[t] = NONE;
    if (pointed_at[r] == pointed_at[r + ONE] || s >= filled[r]) return;
    int listed = NONE;
    for (int k = pointable_at[r]; k < pointable_at[r + ONE]; k++) {
        if (pointable[k] == s) listed = ONE;
    }
    if (!listed) return;
    const float *p = pointed_part + b * item;
    const float *e = e_all + t * item;
    float score = NONE;
    for (int i = NONE; i < item; i++) score += p[i] * e[i];
    g[t] = score;
}

extern "C" __global__ void pointed(float *g, float *loss, int *right, const int *pointable, const int *pointable_at, const int *pointed_list, const int *pointed_at, const int *filled, const int *order, int offset, int n, int slots, float smallest) {
    int b = thread_of();
    if (b >= n) return;
    int r = order[offset + b];
    float *gb = g + b * slots;
    int from = pointed_at[r], to = pointed_at[r + ONE];
    if (from == to) return;
    int any = NONE;
    float top = NONE;
    int best = NONE;
    for (int k = pointable_at[r]; k < pointable_at[r + ONE]; k++) {
        int s = pointable[k];
        if (s >= filled[r]) continue;
        float score = gb[s];
        if (!any || score >= top) {
            top = score;
            best = s;
        }
        any = ONE;
    }
    if (!any) return;
    if (!is_pointed(pointed_list, from, to, best)) right[offset + b] = NONE;
    float total = NONE, hit = NONE;
    for (int k = pointable_at[r]; k < pointable_at[r + ONE]; k++) {
        int s = pointable[k];
        if (s < filled[r]) total += exp(gb[s] - top);
    }
    for (int k = pointable_at[r]; k < pointable_at[r + ONE]; k++) {
        int s = pointable[k];
        if (s < filled[r] && is_pointed(pointed_list, from, to, s)) hit += exp(gb[s] - top) / total;
    }
    hit = hit > smallest ? hit : smallest;
    for (int k = pointable_at[r]; k < pointable_at[r + ONE]; k++) {
        int s = pointable[k];
        if (s >= filled[r]) continue;
        float p = exp(gb[s] - top) / total;
        gb[s] = p - (is_pointed(pointed_list, from, to, s) ? p / hit : NONE);
    }
    loss[b] += -log(hit);
}

extern "C" __global__ void hidden_back(float *d_pre, const float *d_out, const float *g, const float *pre, const float *e_all, const float *out, const float *point, const int *filled, const int *order, int offset, int n, int slots, int item, int h, int c_n, int pointing) {
    int t = thread_of();
    if (t >= n * h) return;
    int b = t / h, j = t % h, r = order[offset + b];
    if (!(pre[t] > NONE)) {
        d_pre[t] = NONE;
        return;
    }
    float dh = NONE;
    const float *d = d_out + b * c_n;
    for (int c = NONE; c < c_n; c++) dh += d[c] * out[c * h + j];
    if (pointing) {
        const float *w = point + j * item;
        for (int s = NONE; s < filled[r]; s++) {
            float gs = g[b * slots + s];
            if (gs == NONE) continue;
            const float *e = e_all + (b * slots + s) * item;
            float x = NONE;
            for (int i = NONE; i < item; i++) x += w[i] * e[i];
            dh += gs * x;
        }
    }
    d_pre[t] = dh;
}

extern "C" __global__ void out_step(float *out, float *out_sq, float *out_bias, float *bias_sq, const float *d_out, const float *pre, int n, int h, int c_n, float rate, float least) {
    int t = thread_of();
    if (t >= c_n * h) return;
    int c = t / h, j = t - c * h;
    float sum = NONE;
    for (int b = NONE; b < n; b++) sum += d_out[b * c_n + c] * rectified(pre[b * h + j]);
    adagrad(out + t, out_sq + t, sum, rate, least);
    if (j == NONE) {
        float total = NONE;
        for (int b = NONE; b < n; b++) total += d_out[b * c_n + c];
        adagrad(out_bias + c, bias_sq + c, total, rate, least);
    }
}

extern "C" __global__ void bias_step(float *bias, float *sq, const float *d_pre, int n, int h, float rate, float least) {
    int j = thread_of();
    if (j >= h) return;
    float sum = NONE;
    for (int b = NONE; b < n; b++) sum += d_pre[b * h + j];
    adagrad(bias + j, sq + j, sum, rate, least);
}

extern "C" __global__ void slot_step(float *weights, float *weights_sq, float *fill, float *fill_sq, const float *d_pre, const float *e_all, const int *filled, const int *order, int offset, int n, int reach, int slots, int item, int h, float rate, float least) {
    int t = thread_of();
    if (t >= reach * h) return;
    int s = t / h, j = t % h;
    float sum = NONE;
    for (int b = NONE; b < n; b++) {
        if (s < filled[order[offset + b]]) sum += d_pre[b * h + j];
    }
    adagrad(fill + t, fill_sq + t, sum, rate, least);
}

extern "C" __global__ void point_weighted(float *weighted, const float *g, const float *e_all, const int *filled, const int *order, int offset, int n, int slots, int item) {
    int t = thread_of();
    if (t >= n * item) return;
    int b = t / item, i = t % item, r = order[offset + b];
    float sum = NONE;
    for (int s = NONE; s < filled[r]; s++) sum += g[b * slots + s] * e_all[(b * slots + s) * item + i];
    weighted[t] = sum;
}

extern "C" __global__ void table_sum(float *sums, int *touched, float *total, const float *d_e, const float *loss, const int *ids, const int *slot_ids, const int *row_slots, const int *places, const int *hot_of, const int *hot_ids, const int *filled, const int *order, int offset, int n, int slots, int item, int mark) {
    __shared__ float gathered[HOT * ITEM];
    int within = threadIdx.x, width = blockDim.x;
    int t = blockIdx.x * width + within;
    for (int k = within; k < HOT * ITEM; k += width) gathered[k] = NONE;
    __syncthreads();
    if (t == NONE) {
        for (int b = NONE; b < n; b++) total[NONE] += loss[b];
    }
    int b = t / slots;
    int s = t - b * slots;
    if (b < n) {
        int r = order[offset + b];
        if (s < filled[r]) {
            int q = row_slots[r] + filled[r] - ONE - s;
            const float *d = d_e + (b * slots + s) * item;
            for (int k = slot_ids[q] - ONE; k < slot_ids[q + ONE]; k++) {
                int id = k < slot_ids[q] ? places[s] : ids[k];
                if (id < NONE) continue;
                int h = hot_of[id];
                if (h >= NONE) {
                    float *m = gathered + h * item;
                    for (int i = NONE; i < item; i++) atomicAdd(m + i, d[i]);
                } else {
                    float *m = sums + id * item;
                    for (int i = NONE; i < item; i++) atomicAdd(m + i, d[i]);
                    touched[id] = mark;
                }
            }
        }
    }
    __syncthreads();
    for (int k = within; k < HOT * item; k += width) {
        float g = gathered[k];
        if (g != NONE) {
            int id = hot_ids[k / item];
            atomicAdd(sums + id * item + k % item, g);
            touched[id] = mark;
        }
    }
}

extern "C" __global__ void table_apply(float *table, float *sq, float *sums, int *touched, const int *ids, const int *slot_ids, const int *row_slots, const int *places, const int *filled, const int *order, int offset, int n, int slots, int item, int mark, float rate, float least) {
    int t = thread_of();
    int b = t / slots;
    int s = t - b * slots;
    if (b >= n) return;
    int r = order[offset + b];
    if (s >= filled[r]) return;
    int q = row_slots[r] + filled[r] - ONE - s;
    for (int k = slot_ids[q] - ONE; k < slot_ids[q + ONE]; k++) {
        int id = k < slot_ids[q] ? places[s] : ids[k];
        if (id < NONE || atomicCAS(touched + id, mark, -mark) != mark) continue;
        for (int i = NONE; i < item; i++) {
            adagrad(table + id * item + i, sq + id * item + i, sums[id * item + i], rate, least);
            sums[id * item + i] = NONE;
        }
    }
}
"#;
because!(
    TRAIN_BODY,
    TrainingMath,
    "the kernels of a training step in one text: the embeddings of a step's slots, the hidden units, the outputs with their softmax loss, the pointing loss, the gradients reaching the hidden units and the embeddings, and AdaGrad on every weight group; a number with no gradient is left as it is, as on the processor, and nothing, one and the smallest positive number a loss is kept above all come in from the program"
);

pub fn train_text(item: usize) -> String {
    format!("#define NONE {}
#define ONE {}
#define TILE {}
#define PER {}
#define SIDE {}
#define HOT {}
#define ITEM {}
{TRAIN_BODY}", 0, 1, crate::steps::TILE, crate::steps::PER, crate::steps::SIDE, crate::steps::HOT, item)
}
because!(train_text, TrainingMath, "the training kernels' text with nothing, one, the tile side, the results a thread works out each way, the side of a block's tile, how many hot feature places a block gathers and the embedding width defined from the program's own numbers and the network");
