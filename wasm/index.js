(function () {
    'use strict';

    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) { return heap[idx]; }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) wasm.__wbindgen_export_0.get(dtor)(a, state.b);
                else state.a = a;
            }
        };
        real.original = state;
        return real;
    }
    function __wbg_adapter_14(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures__invoke1_mut__h0746bc00e3cd9df5(arg0, arg1, addHeapObject(arg2));
    }

    let stack_pointer = 32;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }
    function __wbg_adapter_17(arg0, arg1, arg2) {
        try {
            wasm.wasm_bindgen__convert__closures__invoke1_mut_ref__hb8f9ced51f97e852(arg0, arg1, addBorrowedObject(arg2));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function handleError(f) {
        return function () {
            try {
                return f.apply(this, arguments);

            } catch (e) {
                wasm.__wbindgen_exn_store(addHeapObject(e));
            }
        };
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    let WASM_VECTOR_LEN = 0;

    let cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
        : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachegetInt32Memory0 = null;
    function getInt32Memory0() {
        if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory0;
    }

    async function load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {

            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {

            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    async function init(input) {
        if (typeof input === 'undefined') {
            input = (document.currentScript && document.currentScript.src || new URL('index.js', document.baseURI).href).replace(/\.js$/, '_bg.wasm');
        }
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_removeEventListener_ac0300042557d8c8 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
        });
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = takeObject(arg0).original;
            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }
            var ret = false;
            return ret;
        };
        imports.wbg.__wbg_querySelector_696cb1e67232fbe7 = handleError(function(arg0, arg1, arg2) {
            var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        });
        imports.wbg.__wbg_instanceof_HtmlInputElement_847b0961185c6e6a = function(arg0) {
            var ret = getObject(arg0) instanceof HTMLInputElement;
            return ret;
        };
        imports.wbg.__wbg_getElementById_35de356b82960e7f = function(arg0, arg1, arg2) {
            var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_instanceof_HtmlElement_67a9589b0f1c5c31 = function(arg0) {
            var ret = getObject(arg0) instanceof HTMLElement;
            return ret;
        };
        imports.wbg.__wbg_value_8ed9d1cd27bda7ca = function(arg0, arg1) {
            var ret = getObject(arg1).value;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_setinnerText_9eb84948bb9b7d51 = function(arg0, arg1, arg2) {
            getObject(arg0).innerText = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__wbg_localStorage_62f777b1184f4bea = handleError(function(arg0) {
            var ret = getObject(arg0).localStorage;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        });
        imports.wbg.__wbg_set_3270e87eac61c3e8 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0)[getStringFromWasm0(arg1, arg2)] = getStringFromWasm0(arg3, arg4);
        });
        imports.wbg.__wbg_style_08224ec396c218e8 = function(arg0) {
            var ret = getObject(arg0).style;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_setProperty_3090dd7650e67dd9 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        });
        imports.wbg.__wbg_instanceof_Element_b2ea0d558fc7d331 = function(arg0) {
            var ret = getObject(arg0) instanceof Element;
            return ret;
        };
        imports.wbg.__wbg_remove_962381718003cfd7 = function(arg0) {
            getObject(arg0).remove();
        };
        imports.wbg.__wbg_createElement_2a0e776394ab0fae = handleError(function(arg0, arg1, arg2) {
            var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_instanceof_HtmlTemplateElement_e835f13d3dee39ee = function(arg0) {
            var ret = getObject(arg0) instanceof HTMLTemplateElement;
            return ret;
        };
        imports.wbg.__wbg_setinnerHTML_f91b874a4e14e714 = function(arg0, arg1, arg2) {
            getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__wbg_content_17cbc05e3b7a1bed = function(arg0) {
            var ret = getObject(arg0).content;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_prepend_a09b5868df824922 = handleError(function(arg0, arg1) {
            getObject(arg0).prepend(getObject(arg1));
        });
        imports.wbg.__wbg_classList_262889e9fed65a4e = function(arg0) {
            var ret = getObject(arg0).classList;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_remove_502a44693e55f598 = handleError(function(arg0, arg1, arg2) {
            getObject(arg0).remove(getStringFromWasm0(arg1, arg2));
        });
        imports.wbg.__wbg_add_1b19fef70ac6da51 = handleError(function(arg0, arg1, arg2) {
            getObject(arg0).add(getStringFromWasm0(arg1, arg2));
        });
        imports.wbg.__wbg_setchecked_55afa5312070d694 = function(arg0, arg1) {
            getObject(arg0).checked = arg1 !== 0;
        };
        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            var ret = getObject(arg0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_setvalue_dc83a8a8a01a2b8d = function(arg0, arg1, arg2) {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__wbg_focus_2617465192028e1c = handleError(function(arg0) {
            getObject(arg0).focus();
        });
        imports.wbg.__wbg_instanceof_KeyboardEvent_b4c54a7c0c644df3 = function(arg0) {
            var ret = getObject(arg0) instanceof KeyboardEvent;
            return ret;
        };
        imports.wbg.__wbg_addEventListener_0b521bd333479b91 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
        });
        imports.wbg.__wbg_location_9b266239ee5a222a = function(arg0) {
            var ret = getObject(arg0).location;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_hash_807752e2195bb755 = handleError(function(arg0, arg1) {
            var ret = getObject(arg1).hash;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        });
        imports.wbg.__wbg_instanceof_MouseEvent_55075db8cbe3d2b1 = function(arg0) {
            var ret = getObject(arg0) instanceof MouseEvent;
            return ret;
        };
        imports.wbg.__wbg_instanceof_EventTarget_6a4bb51d820d4a32 = function(arg0) {
            var ret = getObject(arg0) instanceof EventTarget;
            return ret;
        };
        imports.wbg.__wbg_document_76c349f54c28c8fa = function(arg0) {
            var ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_body_ca5c7fc933a74206 = function(arg0) {
            var ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_appendChild_5a186a381c8fff5b = handleError(function(arg0, arg1) {
            var ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_get_1ed08215387a100d = handleError(function(arg0, arg1, arg2, arg3) {
            var ret = getObject(arg1)[getStringFromWasm0(arg2, arg3)];
            var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        });
        imports.wbg.__wbg_key_0d736f3a5f2ca002 = function(arg0, arg1) {
            var ret = getObject(arg1).key;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_target_296eba31ead5b0a6 = function(arg0) {
            var ret = getObject(arg0).target;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_getElementsByClassName_f0225e56eb568cc8 = function(arg0, arg1, arg2) {
            var ret = getObject(arg0).getElementsByClassName(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_length_f5f303fe140c6077 = function(arg0) {
            var ret = getObject(arg0).length;
            return ret;
        };
        imports.wbg.__wbg_item_d9d20caa1c0ff1b9 = function(arg0, arg1) {
            var ret = getObject(arg0).item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_new_17a08b876c4dedc9 = function() {
            var ret = new Object();
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            var ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_set_ede434d91072bd5f = handleError(function(arg0, arg1, arg2) {
            var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        });
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbg_then_45c887a50a229274 = function(arg0, arg1) {
            var ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_resolve_57cbe6ab0b3b60a7 = function(arg0) {
            var ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_self_d1b58dbab69d5bb1 = handleError(function() {
            var ret = self.self;
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_window_de445cb18819ad4b = handleError(function() {
            var ret = window.window;
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_globalThis_68afcb0d98f0112d = handleError(function() {
            var ret = globalThis.globalThis;
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_global_baed4e4fa850c0d0 = handleError(function() {
            var ret = global.global;
            return addHeapObject(ret);
        });
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            var ret = getObject(arg0) === undefined;
            return ret;
        };
        imports.wbg.__wbg_newnoargs_db0587fa712f9acc = function(arg0, arg1) {
            var ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_call_79ca0d435495a83a = handleError(function(arg0, arg1) {
            var ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_instanceof_Window_0e8decd0a6179699 = function(arg0) {
            var ret = getObject(arg0) instanceof Window;
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper1034 = function(arg0, arg1, arg2) {
            var ret = makeMutClosure(arg0, arg1, 13, __wbg_adapter_14);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_closure_wrapper314 = function(arg0, arg1, arg2) {
            var ret = makeMutClosure(arg0, arg1, 13, __wbg_adapter_17);
            return addHeapObject(ret);
        };

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        const { instance, module } = await load(await input, imports);

        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    }

    init("wasm/app.wasm").catch(console.error);

}());
