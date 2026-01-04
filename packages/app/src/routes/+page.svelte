<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";

  interface LogMessage {
    message: string;
    log_type: string;
    timestamp: string;
  }

  let logs = $state<LogMessage[]>([]);
  let isRunning = $state(false);
  let statusMessage = $state("");
  let autoScroll = $state(true);
  let logContainer: HTMLDivElement;
  let unlisten: (() => void) | null = null;
  let statusInterval: ReturnType<typeof setInterval> | null = null;

  onMount(async () => {
    // Listen for log messages from Rust
    unlisten = await listen<LogMessage>("log-message", (event) => {
      logs = [...logs, event.payload];
      if (autoScroll && logContainer) {
        setTimeout(() => {
          logContainer.scrollTop = logContainer.scrollHeight;
        }, 10);
      }
    });

    // Check initial service status
    await checkStatus();
    
    // Poll service status every 2 seconds
    statusInterval = setInterval(async () => {
      await checkStatus();
    }, 2000);
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
    if (statusInterval) {
      clearInterval(statusInterval);
    }
  });

  async function checkStatus() {
    try {
      const running = await invoke<boolean>("is_service_running");
      isRunning = running;
    } catch (error) {
      console.error("Failed to check status:", error);
    }
  }

  async function startService() {
    try {
      statusMessage = "Starting service...";
      const result = await invoke<string>("start_service");
      statusMessage = result;
      await checkStatus();
    } catch (error) {
      statusMessage = `Error: ${error}`;
      logs = [...logs, {
        message: `Failed to start: ${error}`,
        log_type: "ERROR",
        timestamp: new Date().toISOString()
      }];
    }
  }

  async function stopService() {
    try {
      statusMessage = "Stopping service...";
      const result = await invoke<string>("stop_service");
      statusMessage = result;
      await checkStatus();
    } catch (error) {
      statusMessage = `Error: ${error}`;
      logs = [...logs, {
        message: `Failed to stop: ${error}`,
        log_type: "ERROR",
        timestamp: new Date().toISOString()
      }];
    }
  }

  async function restartService() {
    try {
      statusMessage = "Restarting service...";
      const result = await invoke<string>("restart_service");
      statusMessage = result;
      await checkStatus();
    } catch (error) {
      statusMessage = `Error: ${error}`;
      logs = [...logs, {
        message: `Failed to restart: ${error}`,
        log_type: "ERROR",
        timestamp: new Date().toISOString()
      }];
    }
  }

  function clearLogs() {
    logs = [];
  }

  function getLogClass(logType: string): string {
    switch (logType) {
      case "ERROR": return "log-error";
      case "WARN": return "log-warn";
      case "INFO": return "log-info";
      case "EVENT": return "log-event";
      case "SEND": return "log-send";
      default: return "";
    }
  }
</script>

<main class="container">
  <div class="logs-container" bind:this={logContainer}>
    {#each logs as log}
        <div class="log-entry {getLogClass(log.log_type)}">
          <span class="log-timestamp">{log.timestamp}</span>
          <span class="log-type">[{log.log_type}]</span>
          <span class="log-message">{log.message}</span>
        </div>
      {/each}
  </div>
</main>

<style>
  :root {
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.5;
    color: #e0e0e0;
    background-color: #1e1e1e;
  }

  .container {
    margin: 0;
    padding: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }

  .logs-container {
    flex: 1;
    overflow-y: auto;
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  }

  .no-logs {
    color: #757575;
    text-align: center;
    padding: 40px;
  }

  .log-entry {
    padding: 2px 0;
    border-bottom: 1px solid #2d2d2d;
    display: flex;
    gap: 10px;
  }

  .log-timestamp {
    color: #757575;
    white-space: nowrap;
  }

  .log-type {
    font-weight: bold;
    min-width: 60px;
  }

  .log-message {
    flex: 1;
    word-wrap: break-word;
  }

  .log-info .log-type {
    color: #4fc3f7;
  }

  .log-warn .log-type {
    color: #ffb74d;
  }

  .log-error .log-type {
    color: #f44336;
  }

  .log-event .log-type {
    color: #ba68c8;
  }

  .log-send .log-type {
    color: #81c784;
  }

  /* Scrollbar styling */
  .logs-container::-webkit-scrollbar {
    width: 12px;
  }

  .logs-container::-webkit-scrollbar-track {
    background: #1e1e1e;
  }

  .logs-container::-webkit-scrollbar-thumb {
    background: #3d3d3d;
    border-radius: 6px;
  }

  .logs-container::-webkit-scrollbar-thumb:hover {
    background: #4d4d4d;
  }
</style>

