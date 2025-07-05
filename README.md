# TODOs

## todo
great and for the ram usuage the number displayed should be actual GBs and at the top of that column it should show the total GB for that card

## doing

## done
- Modify layout to show all GPUs with each GPU's charts on a single row
- Double the height for better visibility
- Replace random data with nvml_wrapper for real GPU usage
- Add GPU memory usage bars alongside GPU usage bars
- Fix display issues with bar rendering and add "press q to quit" text at bottom
- Implement a random number generator as the data source for the bar charts
- Render the bar charts in the terminal and update them in real time
- Limit the chart to a configurable number of bars wide (x bars)
- Start with 1 bar wide, allow dynamic resizing (e.g., with arrow keys)
- Make each bar thicker (draw multiple columns per bar)
- Refactor rendering to use a more advanced terminal control library (e.g., termion or tui-rs) for efficient, flicker-free updates
- Fix exit handling so Ctrl+C and 'q' both work reliably
- Make each bar stretch horizontally to fill the available width when its value is 100
