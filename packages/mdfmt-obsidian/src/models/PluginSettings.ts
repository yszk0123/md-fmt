export interface PluginSettings {
  ignore: string[];
  formatOnSave: boolean;
  debug: boolean;
}

export const DEFAULT_PLUGIN_SETTINGS: PluginSettings = {
  ignore: [],
  formatOnSave: false,
  debug: false,
};
