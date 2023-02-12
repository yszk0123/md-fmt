import { App, PluginSettingTab, Setting } from 'obsidian';

import { MetadataPlugin } from '~/types';

export class MdfmtPluginSettingTab extends PluginSettingTab {
  plugin: MetadataPlugin;

  constructor(app: App, plugin: MetadataPlugin) {
    super(app, plugin);
    this.plugin = plugin;
  }

  display(): void {
    this.containerEl.empty();

    this.containerEl.createEl('h2', {
      text: 'Metadata plugin settings.',
    });

    new Setting(this.containerEl).setName('Tags').addText((text) =>
      text
        .setPlaceholder('Ignore tags')
        .setValue(this.plugin.settings.ignore.join(','))
        .onChange(async (value) => {
          this.plugin.settings.ignore = value.split(/\s*,\s*/).filter(Boolean);
          await this.plugin.saveSettings();
        })
    );

    new Setting(this.containerEl).setName('FormatOnSave').addToggle((toggle) =>
      toggle.setValue(this.plugin.settings.formatOnSave).onChange(async (value) => {
        this.plugin.settings.formatOnSave = value;
        await this.plugin.saveSettings();
      })
    );

    new Setting(this.containerEl).setName('Debug').addToggle((toggle) =>
      toggle.setValue(this.plugin.settings.debug).onChange(async (value) => {
        this.plugin.settings.debug = value;
        await this.plugin.saveSettings();
      })
    );
  }
}
