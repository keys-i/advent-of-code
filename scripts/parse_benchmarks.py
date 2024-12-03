import re
import sys
import subprocess
import os
from collections import defaultdict

def list_bench_files(benches_folder):
    """List all bench filenames in the benches folder and extract year numbers."""
    bench_files = []
    for file in os.listdir(benches_folder):
        match = re.match(r"year(\d+)\.rs", file)
        if match:
            year = match.group(1)
            bench_files.append(year)
    return bench_files

def run_cargo_bench(year):
    """Run cargo bench for the specified year and return the output."""
    try:
        result = subprocess.run(
            ["cargo", "bench", "--bench", f"year{year}"],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            check=True,
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running cargo bench for year {year}: {e.output}")
        sys.exit(1)

def parse_benchmark_output(benchmark_output, year):
    """Parse the benchmark output and group data by year and day."""
    lines = benchmark_output.splitlines()
    benchmarks = defaultdict(dict)  # {day: {part: time}}

    for line in lines:
        # Match benchmark timing lines
        match = re.match(
            r"(day\d+ part \d+)\s+time:\s+\[\s*([\d\.]+)\s*µs\s+([\d\.]+)\s*µs\s+([\d\.]+)\s*µs\s*\]",
            line,
        )
        if match:
            task, low, mid, high = match.groups()
            day, part = task.split(" part ")
            benchmarks[day][f"part{part}"] = mid

    return {year: benchmarks}

def generate_markdown_table(benchmarks):
    """Generate Markdown tables grouped by year from parsed benchmark data."""
    markdown = ""

    for year, days in sorted(benchmarks.items()):
        markdown += f"### Year {year}\n\n"
        markdown += "| Day Number | Part 1 Time (µs) | Part 2 Time (µs) | Total Time (µs) |\n"
        markdown += "|------------|------------------|------------------|-----------------|\n"

        for day, parts in sorted(days.items()):
            part1_time = parts.get("part1", "N/A")
            part2_time = parts.get("part2", "N/A")
            total_time = "N/A"
            if part1_time != "N/A" and part2_time != "N/A":
                total_time = f"{float(part1_time) + float(part2_time):.2f}"
            markdown += f"| {day.replace('day', '')}          | {part1_time}           | {part2_time}           | {total_time}           |\n"
        markdown += "\n"

    return markdown

def update_readme(benchmarks, readme_file):
    """Update the README.md file with new benchmark tables."""
    with open(readme_file, "r") as f:
        readme_content = f.read()

    # Generate new tables grouped by year
    markdown_tables = generate_markdown_table(benchmarks)
    updated_readme = re.sub(
        r"<!-- BENCHMARK_RESULTS -->.*?<!-- END_BENCHMARK_RESULTS -->",
        f"<!-- BENCHMARK_RESULTS -->\n{markdown_tables}<!-- END_BENCHMARK_RESULTS -->",
        readme_content,
        flags=re.DOTALL,
    )

    with open(readme_file, "w") as f:
        f.write(updated_readme)

if __name__ == "__main__":
    benches_folder = "benches"  # Path to the benches folder
    readme_file = "README.md"  # Path to the README file

    # List all years from bench files
    years = list_bench_files(benches_folder)

    all_benchmarks = {}
    for year in years:
        print(f"Running benchmarks for year {year}...")
        benchmark_output = run_cargo_bench(year)
        benchmarks = parse_benchmark_output(benchmark_output, year)
        all_benchmarks.update(benchmarks)

    # Update README with all benchmark results
    update_readme(all_benchmarks, readme_file)
