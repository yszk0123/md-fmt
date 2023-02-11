export interface PluginSettings {
  ignore: string[];
  debug: boolean;
}

export const DEFAULT_PLUGIN_SETTINGS: PluginSettings = {
  ignore: [],
  debug: false,
};
