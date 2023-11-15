
cargo run 100

This command tells the program to capture 100 packets and save them to capture.pcap 

in the captured_packets directory within your project.

The directory will be auto-created while capturing the packets. 

You can set your own numbers at your own risk.

Afterward you can just fire up WireShark and have a look at the gathered packets. 

Issues: 
Steps to Resolve wpcap.lib Linking Error on Windows:
Install Npcap:

Download and install Npcap from Npcap's official website.
Make sure to install it with the "Install Npcap in WinPcap API-compatible Mode" option.
Download the Npcap SDK:

Get the Npcap SDK from Npcap's SDK download page. The SDK contains the necessary library files, including wpcap.lib.
Set LIB Environment Variable:

Add the path to the Npcap SDK's /Lib or /Lib/x64 folder to your LIB environment variable.
To do this, right-click on 'This PC' or 'My Computer', select 'Properties', then 'Advanced system settings', and click on 'Environment Variables'. Under 'System variables', find or create the LIB variable and add the path to the Npcap SDK library.
If you're using the command line, you can temporarily set the environment variable for your current session with a command like set LIB=%LIB%;C:\Path\To\NpcapSDK\Lib\x64.
Rebuild Your Rust Project:

Run cargo clean and cargo build to rebuild your project with the new environment settings.

Additional Tips:
Restart Required: Sometimes, after changing environment variables, a system restart may be necessary for the changes to take effect.
Check Paths: Ensure the paths are correctly specified, and the relevant files exist in those directories.
Administrative Privileges: Running packet capture operations often requires administrative privileges. Make sure to run your Rust application with the necessary permissions.
This approach should generally resolve issues related to linking wpcap.lib in Rust projects that use packet capturing on Windows. 
