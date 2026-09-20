"""The table: each model, what it holds, what one answer takes, and how much of the item set it answers.

Every model is scored by the one rule in score.py, on the one item set in out/items.jsonl.
"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import score

HERE = os.path.dirname(os.path.abspath(__file__)) + "/out/"
SHOWN = [
    ("dam1", "DAM1"),
    ("qwen25-05b", "Qwen2.5-0.5B-Instruct"),
    ("qwen3-06b", "Qwen3-0.6B"),
    ("lfm2-350m", "LFM2-350M"),
    ("smollm2-360m", "SmolLM2-360M-Instruct"),
    ("smollm2-135m", "SmolLM2-135M-Instruct"),
]


def read(tag):
    path = HERE + f"replies-{tag}.jsonl"
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as file:
        return [json.loads(line) for line in file if line.strip()]


def facts(tag):
    path = HERE + f"facts-{tag}.json"
    if not os.path.exists(path):
        return {}
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def main():
    rows = []
    for tag, name in SHOWN:
        kept = read(tag)
        if kept is None:
            print(f"(no replies for {tag})")
            continue
        fact = facts(tag)
        rows.append({
            "tag": tag, "name": name, "items": len(kept),
            "right": sum(1 for item in kept if score.answered(item["reply"], item["answer"])),
            "exact": sum(1 for item in kept if score.answered(item["reply"], item["answer"], kind=False)),
            "parameters": fact.get("parameters"), "alone": fact.get("one_at_a_time"),
            "weights": fact.get("weights"), "memory": fact.get("memory"),
            "download": fact.get("download", fact.get("weights")),
        })
    rows.sort(key=lambda row: -row["right"])
    print(f"{'model':26} {'parameters':>14} {'weights':>9} {'memory':>9} {'answered':>10} {'exact':>8} {'ms':>7}")
    for row in rows:
        held = f"{row['parameters']:,}" if row["parameters"] else "?"
        alone = f"{row['alone'] * 1000:.0f}" if row["alone"] else "?"
        size = lambda many: f"{many / 1e6:.0f} MB" if many else "?"
        print(f"{row['name']:26} {held:>14} {size(row['weights']):>9} {size(row['memory']):>9} "
              f"{row['right'] / row['items'] * 100:9.1f}% {row['exact'] / row['items'] * 100:7.1f}% {alone:>7}")
    with open(HERE + "table.json", "w", encoding="utf-8") as file:
        json.dump(rows, file, indent=1)

    kept = {row["tag"]: read(row["tag"]) for row in rows}
    folders = sorted({item["folder"] for item in next(iter(kept.values()))}) if kept else []
    print()
    print("lessons".ljust(14) + " ".join(row["tag"][:13].rjust(14) for row in rows))
    for folder in folders:
        line = folder.ljust(14)
        for row in rows:
            part = [item for item in kept[row["tag"]] if item["folder"] == folder]
            got = sum(1 for item in part if score.answered(item["reply"], item["answer"]))
            line += f"{got / max(len(part), 1) * 100:13.1f}%"
        print(line)


main()
