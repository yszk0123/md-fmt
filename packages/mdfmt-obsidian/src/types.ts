import { Plugin } from 'obsidian';

import { PluginSettings } from './models/PluginSettings';

export interface MetadataPlugin extends Plugin {
  settings: PluginSettings;

  loadSettings(): Promise<void>;
  saveSettings(): Promise<void>;
}
