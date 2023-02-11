import { DIFF_DELETE, DIFF_INSERT, diff_match_patch } from 'diff-match-patch';
import { format } from 'mdfmt-js';
import { Editor, EditorPosition } from 'obsidian';

export class FormatAllCommand {
  constructor(private editor: Editor) {}

  run(): void {
    const input = this.editor.getValue();
    const output = format(input);

    getPatches(input, output).forEach(({ text, start, end }) => {
      this.editor.replaceRange(text, start, end);
    });
  }
}

type Patch = { text: string; start: EditorPosition; end?: EditorPosition };

// https://github.com/platers/obsidian-linter/blob/8c055f1fe4148603108158004c15d3e80c8c3496/src/main.ts#L357C4-L381
function getPatches(oldText: string, newText: string): Patch[] {
  const diffMatchPatch = new diff_match_patch();
  const changes = diffMatchPatch.diff_main(oldText, newText);
  const patches: Patch[] = [];

  let currentText = '';
  changes.forEach((change) => {
    const [type, value] = change;

    if (type == DIFF_INSERT) {
      patches.push({ text: value, start: endOfDocument(currentText) });
      currentText += value;
    } else if (type == DIFF_DELETE) {
      const start = endOfDocument(currentText);
      const end = endOfDocument(currentText + value);
      patches.push({ text: '', start, end });
    } else {
      currentText += value;
    }
  });

  return patches;
}

function endOfDocument(doc: string) {
  const lines = doc.split('\n');
  return { line: lines.length - 1, ch: lines[lines.length - 1].length };
}
