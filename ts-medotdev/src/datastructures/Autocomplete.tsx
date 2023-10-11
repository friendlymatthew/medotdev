class TrieNode {
  children: Record<string, TrieNode>;
  isEndOfWord: boolean;

  constructor() {
    this.children = {};
    this.isEndOfWord = false;
  }
}

export class Trie {
  root: TrieNode;

  constructor() {
    this.root = new TrieNode();
  }

  insert(command: string): void {
    let node = this.root;
    for (const char of command) {
      if (!node.children[char]) {
        node.children[char] = new TrieNode();
      }
      node = node.children[char]!;
    }
    node.isEndOfWord = true;
  }

  search(prefix: string): string[] {
    let node: TrieNode = this.root;
    for (const char of prefix) {
      if (!node.children[char]) return [];
      node = node.children[char]!;
    }

    return this._findWordsFromNode(node, prefix);
  }

  isEndOfCommand(command: string): boolean {
    let node = this.root;
    for (const char of command) {
      if (!node.children[char]) return false;
      node = node.children[char]!;
    }
    return node.isEndOfWord;
  }

  private _findWordsFromNode(node: TrieNode, currentWord: string): string[] {
    let words: string[] = [];
    if (node.isEndOfWord) words.push(currentWord);

    for (const char in node.children) {
      const childNode = node.children[char];
      words = words.concat(
        this._findWordsFromNode(childNode!, currentWord + char)
      );
    }

    return words;
  }
}
