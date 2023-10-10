import { format } from 'mdfmt-js';
import { Editor } from 'obsidian';

import { getPatches } from '~/utils/getPatches';

export class FormatAllCommand {
  constructor(private editor: Editor) {}

  run(): void {
    const input = this.editor.getValue();
    const output = format(input);

    const patches = getPatches(input, output);
    for (const patch of patches) {
      this.editor.replaceRange(patch.text, patch.start, patch.end);
    }
  }
}
