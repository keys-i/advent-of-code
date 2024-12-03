import re
import sys

def parse_benchmark_output(benchmark_file):
    """Parse the benchmark output and return a structured dictionary."""
    with open(benchmark_file, "r") as f:
        lines = f.readlines()

    benchmarks = {}
    current_day = None

    for line in lines:
        # Match benchmark timing lines
        match = re.match(r"(day\d+ part \d+)\s+time:\s+\[\s*([\d\.]+)\s*µs\s+([\d\.]+)\s*µs\s+([\d\.]+)\s*µs\s*\]", line)
        if match:
            task, low, mid, high = match.groups()
            day, part = task.split(" part ")
            if day not in benchmarks:
                benchmarks[day] = {}
            benchmarks[day][f"part{part}"] = mid

    return benchmarks


def generate_markdown_table(benchmarks):
    """Generate a Markdown table from parsed benchmark data."""
    table_header = "| Day Number | Part 1 Time (µs) | Part 2 Time (µs) | Total Time (µs) |\n"
    table_header += "|------------|------------------|------------------|-----------------|\n"

    table_rows = []
    for day, parts in sorted(benchmarks.items()):
        part1_time = parts.get("part1", "N/A")
        part2_time = parts.get("part2", "N/A")
        total_time = "N/A"
        if part1_time != "N/A" and part2_time != "N/A":
            total_time = f"{float(part1_time) + float(part2_time):.2f}"
        table_rows.append(f"| {day.replace('day', '')}          | {part1_time}          | {part2_time}          | {total_time}          |")

    return table_header + "\n".join(table_rows)


def update_readme(benchmarks, readme_file):
    """Update the README.md file with the new benchmark table."""
    with open(readme_file, "r") as f:
        readme_content = f.read()

    # Generate new table
    markdown_table = generate_markdown_table(benchmarks)
    updated_readme = re.sub(
        r"<!-- BENCHMARK_RESULTS -->.*?<!-- END_BENCHMARK_RESULTS -->",
        f"<!-- BENCHMARK_RESULTS -->\n{markdown_table}\n<!-- END_BENCHMARK_RESULTS -->",
        readme_content,
        flags=re.DOTALL,
    )

    with open(readme_file, "w") as f:
        f.write(updated_readme)


if __name__ == "__main__":
    benchmark_file = sys.argv[1]  # e.g., "benchmark.txt"
    readme_file = sys.argv[2]    # e.g., "README.md"

    benchmarks = parse_benchmark_output(benchmark_file)
    update_readme(benchmarks, readme_file)
