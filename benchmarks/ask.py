"""Each item asked of one language model: the text, then the question, and the first words it says.

Every model is given the same order and the same two examples, so the shape of an answer is stated
and not guessed, and then the item's text and question. Greedy decoding, so the run repeats. The
reply is cut at its first line.
"""
import json
import os
import sys
import time

import torch
from huggingface_hub import snapshot_download
from transformers import AutoModelForCausalLM, AutoTokenizer

HERE = os.path.dirname(os.path.abspath(__file__)) + "/out/"
ORDER = "\n".join([
    "Answer the question about the text. Reply with the answer only, in as few words as you can.",
    "",
    "Text: the cup is on the table. ben has a pen.",
    "Question: where is the cup?",
    "Answer: table",
    "",
    "Text: mia is in the garden. the garden is small.",
    "Question: is mia in the house?",
    "Answer: no",
    "",
    "Now this one.",
])
NEW = 32
BATCH = 16


def items():
    with open(HERE + "items.jsonl", encoding="utf-8") as file:
        return [json.loads(line) for line in file if line.strip()]


def main():
    name = sys.argv[1]
    tag = sys.argv[2]
    kept = items()
    tokenizer = AutoTokenizer.from_pretrained(name, padding_side="left")
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    model = AutoModelForCausalLM.from_pretrained(name, dtype=torch.float16).to("cuda")
    model.eval()
    chat = tokenizer.chat_template is not None
    prompts = []
    for item in kept:
        asked = ORDER + "\n\nText: " + item["text"] + "\nQuestion: " + item["question"]
        if chat:
            kwargs = {"enable_thinking": False} if "qwen3" in name.lower() else {}
            prompts.append(tokenizer.apply_chat_template(
                [{"role": "user", "content": asked}], tokenize=False, add_generation_prompt=True, **kwargs))
        else:
            prompts.append(asked + "\nAnswer:")
    replies = []
    started = time.perf_counter()
    for at in range(0, len(prompts), BATCH):
        part = prompts[at:at + BATCH]
        taken = tokenizer(part, return_tensors="pt", padding=True).to("cuda")
        with torch.inference_mode():
            made = model.generate(**taken, max_new_tokens=NEW, do_sample=False,
                                  pad_token_id=tokenizer.pad_token_id)
        for row, said in zip(taken["input_ids"], made):
            text = tokenizer.decode(said[len(row):], skip_special_tokens=True)
            said_lines = [one.strip() for one in text.strip().split("\n") if one.strip()]
            first = said_lines[0] if said_lines else ""
            replies.append(first[len("Answer:"):].strip() if first.startswith("Answer:") else first)
        if at % (BATCH * 40) == 0:
            print(f"{at + len(part)} of {len(prompts)}", flush=True)
    seconds = time.perf_counter() - started
    parameters = sum(p.numel() for p in model.parameters())
    # The weights as the hub publishes them, which is what a user downloads.
    folder = snapshot_download(name, allow_patterns=["config.json"])
    weights = sum(os.path.getsize(os.path.join(folder, one)) for one in os.listdir(folder) if one.endswith(".safetensors"))
    # One answer at a time, so the wait for a single question is measured, not the batch's throughput,
    # and the memory of the card is measured over the same pass: what it takes to answer one question.
    torch.cuda.empty_cache()
    torch.cuda.reset_peak_memory_stats()
    alone = []
    for prompt in prompts[:60]:
        taken = tokenizer([prompt], return_tensors="pt").to("cuda")
        torch.cuda.synchronize()
        at = time.perf_counter()
        with torch.inference_mode():
            model.generate(**taken, max_new_tokens=NEW, do_sample=False, pad_token_id=tokenizer.pad_token_id)
        torch.cuda.synchronize()
        alone.append(time.perf_counter() - at)
    alone.sort()
    middle = alone[len(alone) // 2]
    memory = torch.cuda.max_memory_allocated()
    with open(HERE + f"replies-{tag}.jsonl", "w", encoding="utf-8") as file:
        for item, reply in zip(kept, replies):
            file.write(json.dumps({**item, "reply": reply}) + "\n")
    with open(HERE + f"facts-{tag}.json", "w", encoding="utf-8") as file:
        json.dump({"name": name, "parameters": parameters, "weights": weights, "seconds": seconds,
                   "items": len(kept), "per_item": seconds / max(len(kept), 1),
                   "one_at_a_time": middle, "memory": memory}, file)
    print(f"{tag}: {parameters} parameters, {seconds:.1f} s, {seconds / len(kept) * 1000:.0f} ms an item batched, {middle * 1000:.0f} ms alone, {memory / 1e6:.0f} MB on the card")


main()
