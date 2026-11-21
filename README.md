<div>
  <h1>FileMorph</h1>
  <p><b>A desktop file utility built with Tauri and Rust.</b></p>
  
  <h2>What the Code Does</h2>
  <ul>
    <li>Processes local file conversions and utility tasks directly on your machine without relying on external cloud servers, ensuring full data privacy.</li>
    <li>Manages file inputs, validations, and user interactions through a responsive frontend interface.</li>
    <li>Executes fast, memory-efficient file read, write, and transformation operations using a robust Rust systems backend.</li>
    <li>Exposes secure command bridges between the user interface and native system APIs to handle file system paths seamlessly.</li>
  </ul>

  <h2>How It Works Inside the App</h2>
  <ul>
    <li><b>Frontend Interface:</b> The user interacts with a modern desktop window to browse files, select configuration options, and trigger conversion jobs.</li>
    <li><b>Tauri Bridge &amp; IPC:</b> When an action is initiated, asynchronous commands and file parameters are passed securely from the web-based frontend to the underlying Rust layer via Tauri's inter-process communication bridge.</li>
    <li><b>Rust Backend Execution:</b> The Rust core receives the requests, manages multi-threaded execution or streams data safely, processes the files on disk, and reports status updates back to the UI in real-time.</li>
  </ul>

  <h2>Download &amp; Install</h2>
  <p>Grab the ready-to-use Windows setup installers (<code>.msi</code> and <code>.exe</code>) directly from the <a href="../../releases"><b>GitHub Releases</b></a> page.</p>
</div>
