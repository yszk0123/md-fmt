import '~/__workaround__/polyfill.io';

import { load } from 'mdfmt-js';
import { Plugin } from 'obsidian';

import { monkeyPatchConsole } from '~/__workaround__/obsidian-debug-mobile';
import { MdfmtPluginSettingTab } from '~/ui/PluginSettingTab';

import { FormatAllCommand } from './commands/FormatAllCommand';
import { FormatSelectionCommand } from './commands/FormatSelectionCommand';
import { DEFAULT_PLUGIN_SETTINGS, PluginSettings } from './models/PluginSettings';
import { MetadataPlugin } from './types';

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
  }
}
