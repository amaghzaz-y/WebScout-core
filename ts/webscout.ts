import { initSync, WebScout } from "../pkg/webscout";
import wasm from "../pkg/webscout_bg.wasm"

export class WebScoutEngine {
	webscout?: WebScout;
	constructor(index: Uint8Array, tokenizer: Uint8Array, language: string) {
		initSync(wasm);
		this.webscout = new WebScout(language);
		this.webscout.deserialize_index(index);
		this.webscout.deserialize_tokenizer(tokenizer);
		// sets up the query engine, optimizes memory
		this.webscout.setup()
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
}