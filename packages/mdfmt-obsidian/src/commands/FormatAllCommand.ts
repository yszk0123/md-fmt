import { format } from 'mdfmt-js';
import { Editor } from 'obsidian';

export class FormatAllCommand {
  constructor(private editor: Editor) {}

  run(): void {
    const input = this.editor.getValue();
    const output = format(input);
    this.editor.setValue(output);
  }
}
