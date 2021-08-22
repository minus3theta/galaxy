/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/galaxy_bg.wasm": function() {
/******/ 			return {
/******/ 				"./galaxy_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_log_afe3ad486b30ab70": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_log_afe3ad486b30ab70"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_json_serialize": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_json_serialize"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_fetch_d6391b3bc62838b4": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_fetch_d6391b3bc62838b4"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_b99429ec408dcb8d": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_instanceof_Window_b99429ec408dcb8d"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_6d5890b86bbf5b96": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_document_6d5890b86bbf5b96"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementById_f059b7401a23ee7c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_getElementById_f059b7401a23ee7c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_a2acc34cc0a30700": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_a2acc34cc0a30700"](p0i32);
/******/ 					},
/******/ 					"__wbg_width_99577d88b55411fa": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_width_99577d88b55411fa"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_636c7530a04a1f7c": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_height_636c7530a04a1f7c"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_c91489f5e0f738d8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_getContext_c91489f5e0f738d8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_top_c20bec8a03405e53": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_top_c20bec8a03405e53"](p0i32);
/******/ 					},
/******/ 					"__wbg_left_5c2b0bfa52b0d2fd": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_left_5c2b0bfa52b0d2fd"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_EventTarget_2ce9a4918f446d4e": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_instanceof_EventTarget_2ce9a4918f446d4e"](p0i32);
/******/ 					},
/******/ 					"__wbg_addEventListener_957c027f76736fd0": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_addEventListener_957c027f76736fd0"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Response_1ab2f2852729fd69": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_instanceof_Response_1ab2f2852729fd69"](p0i32);
/******/ 					},
/******/ 					"__wbg_url_16ecc8558028ae48": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_url_16ecc8558028ae48"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_status_ac03b4fa1bb5e668": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_status_ac03b4fa1bb5e668"](p0i32);
/******/ 					},
/******/ 					"__wbg_headers_d0e7509aa3d96c81": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_headers_d0e7509aa3d96c81"](p0i32);
/******/ 					},
/******/ 					"__wbg_text_fdd71005335b06f9": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_text_fdd71005335b06f9"](p0i32);
/******/ 					},
/******/ 					"__wbg_getBoundingClientRect_2b34d257f8424b06": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_getBoundingClientRect_2b34d257f8424b06"](p0i32);
/******/ 					},
/******/ 					"__wbg_fetch_40fe4ece1cc2ad3c": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_fetch_40fe4ece1cc2ad3c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_cf60543e642e5a93": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_cf60543e642e5a93"](p0i32);
/******/ 					},
/******/ 					"__wbg_setfillStyle_5cbc4d52520cb206": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_setfillStyle_5cbc4d52520cb206"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_fillRect_798658a3fb2ea875": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_fillRect_798658a3fb2ea875"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_clientX_1f3519e5175f75ac": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_clientX_1f3519e5175f75ac"](p0i32);
/******/ 					},
/******/ 					"__wbg_clientY_3950aa0706a34989": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_clientY_3950aa0706a34989"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_cf4002932f67522d": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_new_cf4002932f67522d"]();
/******/ 					},
/******/ 					"__wbg_append_805e319df958b1f4": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_append_805e319df958b1f4"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_newwithstrandinit_d9d8ffa577544082": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_newwithstrandinit_d9d8ffa577544082"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_is_function": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_is_function"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_1a11e7e8c906996c": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_newnoargs_1a11e7e8c906996c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_is_object": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_is_object"](p0i32);
/******/ 					},
/******/ 					"__wbg_next_a153d72ec9f76de7": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_next_a153d72ec9f76de7"](p0i32);
/******/ 					},
/******/ 					"__wbg_done_1d79fc301127c139": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_done_1d79fc301127c139"](p0i32);
/******/ 					},
/******/ 					"__wbg_value_4c025ad337ce3912": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_value_4c025ad337ce3912"](p0i32);
/******/ 					},
/******/ 					"__wbg_iterator_18e2d2132471e894": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_iterator_18e2d2132471e894"]();
/******/ 					},
/******/ 					"__wbg_new_4b48f9f8159fea77": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_new_4b48f9f8159fea77"]();
/******/ 					},
/******/ 					"__wbg_call_e91f71ddf1f45cff": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_call_e91f71ddf1f45cff"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_next_f6ffce741b18af05": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_next_f6ffce741b18af05"](p0i32);
/******/ 					},
/******/ 					"__wbg_resolve_7161ec6fd5b1cd29": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_resolve_7161ec6fd5b1cd29"](p0i32);
/******/ 					},
/******/ 					"__wbg_then_6d5072fec3fdb237": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_then_6d5072fec3fdb237"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_then_4f3c7f6f3d36634a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_then_4f3c7f6f3d36634a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_self_b4546ea7b590539e": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_self_b4546ea7b590539e"]();
/******/ 					},
/******/ 					"__wbg_window_c279fea81f426a68": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_window_c279fea81f426a68"]();
/******/ 					},
/******/ 					"__wbg_globalThis_038a6ea0ff17789f": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_globalThis_038a6ea0ff17789f"]();
/******/ 					},
/******/ 					"__wbg_global_4f93ce884bcee597": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_global_4f93ce884bcee597"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_buffer_79a3294266d4e783": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_buffer_79a3294266d4e783"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_22a36e6023ad3cd0": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_newwithbyteoffsetandlength_22a36e6023ad3cd0"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_945397fb09fec0b8": function(p0i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_new_945397fb09fec0b8"](p0i32);
/******/ 					},
/******/ 					"__wbg_get_6d26c712aa73c8b2": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_get_6d26c712aa73c8b2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_has_6beec53675bce86a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_has_6beec53675bce86a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_set_d29a397c9cc5d746": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbg_set_d29a397c9cc5d746"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper324": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_closure_wrapper324"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper519": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/galaxy_bg.js"].exports["__wbindgen_closure_wrapper519"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/galaxy_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/galaxy_bg.wasm":"57b416073bae21294388"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });