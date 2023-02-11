import { format } from 'mdfmt-js';
import { Editor } from 'obsidian';

export class FormatSelectionCommand {
  constructor(private editor: Editor) {}

  run(): void {
    const input = this.editor.getSelection();
    const output = format(input);
    this.editor.replaceSelection(output);
  }
}
