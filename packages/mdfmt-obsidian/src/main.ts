import '~/__workaround__/polyfill.io';

import { load } from 'mdfmt-js';
import { MarkdownView, Plugin } from 'obsidian';

import { monkeyPatchConsole } from '~/__workaround__/obsidian-debug-mobile';
import { MdfmtPluginSettingTab } from '~/ui/PluginSettingTab';

import { FormatAllCommand } from './commands/FormatAllCommand';
import { FormatSelectionCommand } from './commands/FormatSelectionCommand';
import { DEFAULT_PLUGIN_SETTINGS, PluginSettings } from './models/PluginSettings';
import { MetadataPlugin } from './types';

type Unregister = () => void;

const noop = () => {
  /* nothing */
};

export default class MetadataPluginImpl extends Plugin implements MetadataPlugin {
  settings!: PluginSettings;

  async onload() {
    await load();
    await this.loadSettings();

    if (this.settings.debug) {
      monkeyPatchConsole(this);
    }

    this.registerCommands();
    this.registerRibbonIcons();

    this.addSettingTab(new MdfmtPluginSettingTab(this.app, this));
  }

  onunload() {
    /* nothing */
  }

  async loadSettings(): Promise<void> {
    const data = await this.loadData();
    this.settings = { ...DEFAULT_PLUGIN_SETTINGS, ...data };
  }

  async saveSettings() {
    await this.saveData(this.settings);
  }

  private registerRibbonIcons() {
    // this.addRibbonIcon('dice', 'Metadata', (_event: MouseEvent) => {
    //   new Notice('This is a notice!');
    // });
  }

  private registerCommands() {
    this.addCommand({
      id: 'mdfmt-format-all',
      name: 'Format All',
      editorCallback: (editor, _view) => new FormatAllCommand(editor).run(),
    });

    this.addCommand({
      id: 'mdfmt-format-selection',
      name: 'Format Selection',
      editorCallback: (editor, _view) => new FormatSelectionCommand(editor).run(),
    });

    this.register(
      this.insertBeforeCommand('editor:save-file', () => {
        if (!this.settings.formatOnSave) {
          return;
        }

        const activeView = this.app.workspace.getActiveViewOfType(MarkdownView);
        if (!activeView) {
          return;
        }

        new FormatAllCommand(activeView.editor).run();
      })
    );
  }

  private insertBeforeCommand(commandId: 'editor:save-file', callback: () => void): Unregister {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const commandDefinition = (this.app as any).commands?.commands?.[commandId];
    const originalCallback = commandDefinition?.callback;
    if (typeof originalCallback !== 'function') {
      return noop;
    }

    commandDefinition.callback = () => {
      callback();
      originalCallback();
    };

    return () => {
      commandDefinition.callback = originalCallback;
    };
  }
}
