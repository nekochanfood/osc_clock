import { watch } from 'fs';
import { spawn } from 'child_process';
import { resolve, join } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const workspaceRoot = resolve(__dirname, '..');
const iconPath = join(workspaceRoot, 'assets', 'icons', 'icon.svg');

console.log('Watching:', iconPath);
console.log('Press Ctrl+C to stop\n');

let isGenerating = false;

async function generateIcons() {
  if (isGenerating) {
    console.log('[Skipped] Generation already in progress');
    return;
  }

  isGenerating = true;
  const timestamp = new Date().toLocaleTimeString();
  console.log(`\n[${timestamp}] Icon changed, regenerating...`);

  try {
    // Run the Rust icon generator
    const iconGen = spawn('cargo', ['run', '--manifest-path', 'scripts/icon-generator/Cargo.toml'], {
      cwd: workspaceRoot,
      stdio: 'inherit'
    });

    iconGen.on('close', (code) => {
      const endTime = new Date().toLocaleTimeString();
      if (code === 0) {
        console.log(`[${endTime}] ✅ Icons generated successfully`);
      } else {
        console.error(`[${endTime}] ❌ Icon generation failed with code ${code}`);
      }
      isGenerating = false;
    });

    iconGen.on('error', (err) => {
      console.error(`[${new Date().toLocaleTimeString()}] ❌ Failed to start icon generator:`, err.message);
      isGenerating = false;
    });
  } catch (err) {
    console.error(`[${new Date().toLocaleTimeString()}] ❌ Error:`, err);
    isGenerating = false;
  }
}

// Initial generation
console.log('Generating initial icons...');
generateIcons();

// Watch for changes
const watcher = watch(iconPath, (eventType) => {
  if (eventType === 'change') {
    generateIcons();
  }
});

// Handle cleanup
process.on('SIGINT', () => {
  console.log('\n\nStopping icon watcher...');
  watcher.close();
  process.exit(0);
});

process.on('SIGTERM', () => {
  watcher.close();
  process.exit(0);
});

console.log('👀 Watching for changes...');
