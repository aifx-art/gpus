use std::io::{Stdout, Write};
use crossterm::{execute, cursor, style::{Color, Print, SetForegroundColor}, terminal::{Clear, ClearType}};
use tui::widgets::{Block, Borders, BarChart, Paragraph, Gauge};
use tui::style::{Color as TuiColor, Style, Modifier};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::layout::{Rect, Layout, Constraint, Direction};
use crate::data::GpuData;

pub fn draw_bars(stdout: &mut Stdout, data: &[u8]) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;
    for (i, &val) in data.iter().enumerate() {
        // Make each bar thicker by drawing two columns per bar
        let bar = "██".repeat(val as usize);
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("{:2}: {}\n", i, bar)),
        )?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn draw_bars_tui<T: std::io::Write>(terminal: &mut Terminal<CrosstermBackend<T>>, gpu_data: &[GpuData]) -> std::io::Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        
        if gpu_data.is_empty() {
            // Show message when no GPUs are detected
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5), // title area
                    Constraint::Min(5),    // message area
                    Constraint::Length(1), // quit instruction
                ])
                .split(size);
            
            let title = Paragraph::new("GPU Monitor - No GPUs detected or NVML unavailable")
                .style(Style::default().fg(TuiColor::White))
                .block(Block::default().borders(Borders::ALL).title("GPU Monitor"));
            f.render_widget(title, chunks[0]);
            
            let message = Paragraph::new("No GPU data available.\nMake sure NVIDIA drivers and NVML are properly installed.")
                .style(Style::default().fg(TuiColor::Yellow))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(message, chunks[1]);
            
            let quit_text = Paragraph::new("Press 'q' to quit")
                .style(Style::default().fg(TuiColor::Gray));
            f.render_widget(quit_text, chunks[2]);
            return;
        }
        
        // Calculate layout: title + GPU rows + quit instruction
        let gpu_count = gpu_data.len();
        let row_height = 6; // Double the height for better visibility
        let title_height = 3;
        let quit_height = 1;
        
        let mut constraints = vec![Constraint::Length(title_height)];
        for _ in 0..gpu_count {
            constraints.push(Constraint::Length(row_height));
        }
        constraints.push(Constraint::Length(quit_height));
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(size);
        
        // Title
        let title_text = format!("{} GPU{} detected", 
            gpu_count, 
            if gpu_count == 1 { "" } else { "s" }
        );
        
        let title = Paragraph::new(title_text)
            .style(Style::default().fg(TuiColor::White))
            .block(Block::default().borders(Borders::ALL).title("Real-time GPU Monitoring"));
        f.render_widget(title, chunks[0]);
        
        // Render each GPU on its own row
        for (i, gpu) in gpu_data.iter().enumerate() {
            let gpu_chunk = chunks[i + 1]; // +1 to skip title
            
            // Split each GPU row into: name + usage gauge + memory gauge
            let gpu_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(25), // GPU name
                    Constraint::Percentage(37), // Usage gauge
                    Constraint::Percentage(37), // Memory gauge
                ])
                .split(gpu_chunk);
            
            // GPU name and info
            let gpu_info = format!("{}\nUsage: {}%\nMemory: {}%", 
                gpu.name, 
                gpu.usage_percent, 
                gpu.memory_percent
            );
            let name_widget = Paragraph::new(gpu_info)
                .style(Style::default().fg(TuiColor::White))
                .block(Block::default().borders(Borders::ALL).title(format!("GPU {}", i)));
            f.render_widget(name_widget, gpu_layout[0]);
            
            // Usage gauge
            let usage_gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("GPU Usage %"))
                .gauge_style(Style::default().fg(TuiColor::Cyan))
                .percent(gpu.usage_percent as u16)
                .label(format!("{}%", gpu.usage_percent));
            f.render_widget(usage_gauge, gpu_layout[1]);
            
            // Memory gauge
            let memory_gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(format!("Memory Usage {}%", gpu.memory_percent)))
                .gauge_style(Style::default().fg(TuiColor::Green))
                .percent(gpu.memory_percent as u16)
                .label(format!("{:.1} GB / {:.1} GB", gpu.memory_used_gb, gpu.memory_total_gb));
            f.render_widget(memory_gauge, gpu_layout[2]);
        }
        
        // Quit instruction at bottom
        let quit_text = Paragraph::new("Press 'q' to quit")
            .style(Style::default().fg(TuiColor::Gray));
        f.render_widget(quit_text, chunks[chunks.len() - 1]);
    })?;
    Ok(())
}
