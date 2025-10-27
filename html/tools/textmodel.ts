type Attrs = { [key: string]: string } & { style?: CSSStyleDeclaration };

function createElement<E extends HTMLElement>(
	name: string,
	content: string | HTMLElement[] | HTMLElement = [],
	attributes: Attrs = {},
): E {
	const el = document.createElement(name);
	for (const prop in attributes) {
		const attr = attributes[prop] as CSSStyleDeclaration | string;
		if (typeof attr === "string") {
			el.setAttribute(prop, attr);
		} else {
			for (const stl in attr) {
				el.style[stl] = attr[stl];
			}
		}
	}
	// Array of elements or single element
	if (Array.isArray(content)) {
		el.append(...content);
	} else if (typeof content === 'string') {
		el.innerText = content
	} else {
		el.append(content);
	}
	return el as E;
}

const $ = (query: string, doc: ParentNode = document) => doc.querySelector(query);
const $$ = (query: string, doc: ParentNode = document) => doc.querySelectorAll(query);

type ModelSlice = | {
	type: 'text',
	text: string,
} | {
	type: 'variable',
	name: string,
} | {
	type: 'optional',
	needs: string[],
	slices: ModelSlice[],
};

type Model = {
	inputs: string[],
	rawModel: string,
	slices: ModelSlice[],
};

function parseModelSlices(text: string): ModelSlice[] {
	type State = {
		type: 'text' | 'variable',
		buffer: string,
	} | {
		type: 'optional',
		until: number,
	};
	let state: State = { type: 'text', buffer: '' };
	const slices: ModelSlice[] = [];

	const pushState = () => {
		if (state.type == 'variable') {
			slices.push({
				type: 'variable',
				name: state.buffer
			});
		} else if (state.type == 'text') {
			slices.push({
				type: 'text',
				text: state.buffer
			});
		}
	};

	let index = 0;
	for (const char of text) {
		if (char == '{' && state.type == 'text') {
			pushState();
			state = {
				type: 'variable',
				buffer: '',
			};
		} else if (char == '}' && state.type == 'variable') {
			pushState();
			state = {
				type: 'text',
				buffer: '',
			};
		} else if (char == '[') {
			pushState();
			const until = text.slice(index).indexOf(']') + index;
			const innerText = text.slice(index + 1, until);
			const innerSlices = parseModelSlices(innerText);
			const needs = innerSlices.filter(s => s.type == 'variable').map(s => s.name);
			slices.push({
				type: 'optional',
				needs,
				slices: innerSlices
			})
			state = {
				type: 'optional',
				until: until,
			}
		} else if (state.type === 'optional') {
			if (state.until == index) {
				state = {
					type: 'text',
					buffer: '',
				};
			}
		} else {
			state.buffer += char;
		}
		index++;
	}

	pushState();
	return slices.filter(a =>
		(a.type === 'text' && a.text) || a.type !== 'text'
	);
}

function parseModel(text: string): Model {
	const inputs = Array.from(text.matchAll(/{(.*?)}/g)).map((match) =>
		match[1]
	);
	return {
		rawModel: text,
		inputs,
		slices: parseModelSlices(text),
	};
}

function renderModel(slices: ModelSlice[], vars: Record<string, string | null>): string {
	const givenVariables = Object.entries(vars)
		.filter(([_, value]) => value !== null)
		.map(([name, _]) => name);
	let buffer = '';

	for (const item of slices) {
		if (item.type == 'variable') {
			buffer += vars[item.name] ?? `{${item.name}}`;
		} else if (item.type == 'text') {
			buffer += item.text;
		} else {
			const can = item.needs.every(i => givenVariables.includes(i));
			if (can || true) {
				const txt = renderModel(item.slices, vars);
				buffer += txt;
			}
		}
	}
	return buffer;
}

window.addEventListener('load', () => {
	const txt = "{SOBRENOME}, {Nome}. {Título}[: {subtítulo}]. [{Edição}. ]{Local de publicação}: {Editora}, {Ano de publicação}";
	const m = parseModel(txt);
	const t = renderModel(m.slices, {
		SOBRENOME: "MANSE",
		Nome: "Pedro",
	});
	console.log(m);
	console.log(t);

	const div = $("#test");
	const out = $("pre");
	const vars = {};
	for (const name of m.inputs) {
		div?.appendChild(createElement("label", `${name}: `, { type: "text", name, }));
		const input = div?.appendChild(createElement<HTMLInputElement>("input", [], { type: "text", name, }));
		div?.appendChild(createElement("br"));
		input?.addEventListener("change", () => {
			vars[name] = input.value;
			out!.innerHTML = renderModel(m.slices, vars);
		});
	}
});


// CBD liquidez diária
