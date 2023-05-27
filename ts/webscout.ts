import { initSync, WebScout } from "../pkg/webscout";
import wasm from "../pkg/webscout_bg.wasm"

export default class WebScoutEngine {
	webscout?: WebScout;
	constructor(index: Uint8Array | null, tokenizer: Uint8Array, language: string) {
		initSync(wasm);
		this.webscout = new WebScout(language);
		if (index !== null) { this.webscout.deserialize_index(index); }
		this.webscout.deserialize_tokenizer(tokenizer);
		// sets up the query engine, optimizes memory
		this.webscout.setup()
	}
	Index(title: string, body: string) {
		this.webscout?.index(title, body)
	}
	Tokenize(word: string): string | undefined {
		return this.webscout?.tokenize(word);
	}
	SearchAll(query: string): any {
		return JSON.parse(this.webscout?.search_all(query));
	}
	Search(query: string): any {
		return JSON.parse(this.webscout?.search_above_average(query));
	}
	ExportIndex(): Uint8Array | undefined {
		return this.webscout?.export_index()
	}
}