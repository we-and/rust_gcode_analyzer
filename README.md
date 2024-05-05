# GcodeAnalyzer

## Introduction
`GcodeAnalyzer` is a Rust-based command-line tool designed to analyze G-code files used in 3D printing. It processes G-code files to identify and report specific characteristics such as slicer type, printing estimates, and potentially malicious commands.

## Features
- **Slicer Detection**: Identifies the slicer used based on specific comments and settings within the G-code.
- **Print Estimate Extraction**: Calculates estimated print times from G-code comments.
- **Material Flushing Detection**: Detects start and end signals for material changes in multi-material prints.
- **Safety Checks**: Scans for commands that could potentially harm the printer or deviate from expected norms.

## Installation

### Prerequisites
- Rust and Cargo (latest stable release recommended)
- Git (optional, for cloning the repository)

### Steps
1. **Clone the repository** (skip if you have the project files):
   ```bash
   git clone https://github.com/yourusername/GcodeAnalyzer.git
   cd GcodeAnalyzer
