# Run Journal CLI

A command-line application written in Rust for tracking running activities. This tool helps runners maintain detailed logs of their training sessions, including distance, duration, pace calculations, and environmental conditions.

## Features

The application captures comprehensive running session data including:

- Run distance in kilometers
- Duration in HH:MM:SS or MM:SS format
- Automatically calculated pace per kilometer
- Rate of perceived exertion (RPE) on a scale of 1-10
- Temperature in Celsius
- Custom comments for each session
- Automatic date stamping
- JSON file storage with daily entries

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- The following dependencies (specified in `Cargo.toml`):
  - serde
  - serde_json
  - chrono
  - std

### Setup

1. Clone the repository:
```bash
git clone [your-repository-url]
cd run-journal
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

## Usage

The application will prompt you for the following information:

1. Total distance (in kilometers)
2. Run duration (in HH:MM:SS or MM:SS format)
3. Rate of exertion (scale of 1-10)
4. Temperature (in Celsius)
5. Additional comments

Example session:
```
Enter in total distance: 21.1
Enter run duration as HH:MM:SS or MM:SS: 1:45:30
Enter rate of exertion (1-10): 8
Enter temperature (Celcius): 22.5
Enter any comments: Half marathon training run, felt strong on hills
```

## Data Storage

Run data is stored in JSON format with the following structure:

```json
{
  "date": "2025-01-25",
  "duration_formatted": "1:45:30",
  "exertion_rate": 8,
  "temperature_celcius": 22.5,
  "comments": "Half marathon training run, felt strong on hills",
  "pace": "5′02″",
  "distance": 21.1
}
```

Files are saved in `/Users/jfk/Documents/runjournal/2025/` with the naming format `run_YYYY-MM-DD.json`.

## Technical Details

### Pace Calculation

The application handles pace calculations with precision, converting all times to seconds first before computing the per-kilometer pace. The pace is displayed using prime (′) and double-prime (″) notation for minutes and seconds respectively.

### Input Validation

- Distance and temperature inputs are validated as floating-point numbers
- Time inputs are validated against HH:MM:SS and MM:SS formats
- RPE inputs are validated to ensure they fall within the 1-10 range

### Error Handling

The application implements comprehensive error handling for:
- Invalid time format inputs
- File system operations
- JSON serialization/deserialization
- Numeric parsing operations

## Future Improvements

Potential enhancements could include:
- Data visualization capabilities
- Running statistics and trends analysis
- Export functionality to common fitness platforms
- Support for different distance units (miles/kilometers)
- Temperature unit conversion
- Customizable file storage location

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

[Your chosen license]
