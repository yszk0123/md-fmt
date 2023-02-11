import { Plugin } from 'obsidian';

// https://gist.github.com/liamcain/3f21f1ee820cb30f18050d2f3ad85f3f
declare module 'obsidian' {
  interface App {
    isMobile: boolean;
  }
}

// Call this method inside your plugin's `onLoad` function
export function monkeyPatchConsole(plugin: Plugin) {
  if (!plugin.app.isMobile) {
    return;
  }

  const logFile = `${plugin.manifest.dir}/logs.txt`;
  const logs: string[] = [];
  const logMessages =
    (prefix: string) =>
    (...messages: unknown[]) => {
      logs.push(`\n[${prefix}]`);
      for (const message of messages) {
        logs.push(String(message));
      }
      plugin.app.vault.adapter.write(logFile, logs.join(' '));
    };

  console.debug = logMessages('debug');
  console.error = logMessages('error');
  console.info = logMessages('info');
  console.log = logMessages('log');
  console.warn = logMessages('warn');
}
