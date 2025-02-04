# Universal GPU Driver  

A simple, open-source tool to manage GPU drivers on Windows. It scans your system, identifies your graphics hardware, and installs or updates drivers automatically. Designed for users who want a hassle-free way to keep their GPU drivers up-to-date.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-gpu-driver/releases) tab.  
2. Download the latest `.zip` file for your Windows version.  
3. Unzip the file and run `UniversalGpuDriver.exe`.  

## Usage  
1. After installation, the tool launches automatically.  
2. Follow the on-screen prompts to scan and install drivers.  
3. Restart your computer when prompted to apply changes.  

## Features  
- **Automatic Detection:** Identifies your GPU model and operating system.  
- **Multi-Brand Support:** Supports every graphics card, including NVIDIA, AMD, Intel, Apple, Matrox, S3 Graphics, 3dfx Interactive, and more.
- **Driver Updates:** Checks for updates and installs the latest stable versions.  
- **Simple GUI:** Easy-to-navigate interface with clear instructions.  
- **Windows Compatibility:** Supports Windows 10 and 11.  
- **Lightweight:** Minimal system resource usage during scans.  
- **Error Recovery:** Restores previous drivers if an update fails.  
- **Offline Mode:** Uses cached drivers if no internet connection is available.  
- **Uninstaller:** Removes old or unused drivers to free up space.  
- **Admin Rights Handling:** Requests permissions only when needed.  
- **Logging:** Saves installation logs for troubleshooting.  

## Build from Source  
For developers or advanced users:  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone this repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-gpu-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-gpu-driver  
   cargo build --release  
   ```  

## Contributing  
Contributions are currently paused due to limited maintenance capacity.  

## License  
MIT License © 2025 muzondie. See [LICENSE](LICENSE) for details.