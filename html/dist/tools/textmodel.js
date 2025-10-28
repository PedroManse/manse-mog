"use strict";
function El(name, content = [], attributes = {}) {
    const el = document.createElement(name);
    for (const prop in attributes) {
        const attr = attributes[prop];
        if (typeof attr === "string") {
            el.setAttribute(prop, attr);
        }
        else {
            for (const stl in attr) {
                el.style[stl] = attr[stl];
            }
        }
    }
    // Array of elements or single element
    if (Array.isArray(content)) {
        el.append(...content);
    }
    else if (typeof content === 'string') {
        el.innerText = content;
    }
    else {
        el.append(content);
    }
    return el;
}
const $ = (query, doc = document) => doc.querySelector(query);
const $$ = (query, doc = document) => doc.querySelectorAll(query);
function snakeCase(text) {
    return text.replaceAll(' ', '_');
}
function parseModelSlices(text) {
    let state = { type: 'text', buffer: '' };
    const slices = [];
    const pushState = () => {
        if (state.type == 'variable') {
            slices.push({
                type: 'variable',
                name: state.buffer
            });
        }
        else if (state.type == 'text') {
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
        }
        else if (char == '}' && state.type == 'variable') {
            pushState();
            state = {
                type: 'text',
                buffer: '',
            };
        }
        else if (char == '[') {
            pushState();
            const until = text.slice(index).indexOf(']') + index;
            const innerText = text.slice(index + 1, until);
            const innerSlices = parseModelSlices(innerText);
            const needs = innerSlices.filter(s => s.type == 'variable').map(s => s.name);
            slices.push({
                type: 'optional',
                needs,
                slices: innerSlices
            });
            state = {
                type: 'optional',
                until: until,
            };
        }
        else if (state.type === 'optional') {
            if (state.until == index) {
                state = {
                    type: 'text',
                    buffer: '',
                };
            }
        }
        else {
            state.buffer += char;
        }
        index++;
    }
    pushState();
    return slices.filter(a => (a.type === 'text' && a.text) || a.type !== 'text');
}
function parseModel(text) {
    const inputs = Array.from(text.matchAll(/{(.*?)}/g)).map((match) => match[1]);
    return {
        rawModel: text,
        inputs,
        slices: parseModelSlices(text),
    };
}
function renderModel(slices, vars) {
    const givenVariables = Object.entries(vars)
        .filter(([_, value]) => value)
        .map(([name, _]) => name);
    let buffer = '';
    for (const item of slices) {
        if (item.type == 'variable') {
            buffer += vars[item.name] ?? `{${item.name}}`;
        }
        else if (item.type == 'text') {
            buffer += item.text;
        }
        else {
            const can = item.needs.every(i => givenVariables.includes(i));
            if (can) {
                const txt = renderModel(item.slices, vars);
                buffer += txt;
            }
        }
    }
    return buffer;
}
window.addEventListener('load', () => {
    const modes = {};
    $$("div#modes > div.mode").forEach(mode => {
        const modeName = mode.getAttribute('name');
        modes[modeName ?? ""] = mode;
        const modeText = $("textarea.model", mode);
        const modeForm = El("form");
        mode.insertAdjacentElement('afterbegin', modeForm);
        // @ts-expect-error maybe i'll fix this some day
        const out = El("textarea", [], { class: 'output', style: { width: "400px" } });
        mode.insertAdjacentElement('beforeend', out);
        const vars = {};
        const m = parseModel(modeText.innerHTML);
        for (const name of m.inputs) {
            const snakeCaseName = snakeCase(name);
            const input = El("input", [], { type: "text", name: snakeCaseName, });
            modeForm.appendChild(El("label", `${name}: `, { type: "text", for: snakeCaseName, }));
            modeForm.appendChild(input);
            modeForm.appendChild(El("br"));
            input.addEventListener("change", () => {
                vars[name] = input.value;
                out.innerHTML = renderModel(m.slices, vars);
            });
        }
    });
    const modeSelector = $("select#mode-selector");
    const modeOptions = Object.keys(modes).map(name => El("option", name, { value: name }));
    let selectedModeName;
    modeSelector.append(...modeOptions);
    modeSelector.addEventListener("change", () => {
        const modeName = modeSelector.value;
        selectedModeName = modeName;
        const selectedMode = modes[modeName];
        Object.values(modes).forEach(m => {
            m.classList.remove('selected');
        });
        selectedMode.classList.add('selected');
    });
    $("button")?.addEventListener('click', () => {
        const selectedMode = modes[selectedModeName];
        console.log($("textarea.output", selectedMode)?.innerHTML);
        navigator.clipboard.writeText($("textarea.output", selectedMode)?.innerHTML ?? '');
    });
});
// CBD liquidez diária
