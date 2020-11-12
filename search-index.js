var searchIndex = JSON.parse('{\
"wasm_language_server":{"doc":"The WebAssembly language server.","i":[[0,"cli","wasm_language_server","Command-line interface for the WASM language server.",null,null],[5,"cli","wasm_language_server::cli","Invokes the command-line interface for the WASM language…",null,[[]]],[0,"core","wasm_language_server","Core functionality for the WASM language server.",null,null],[0,"document","wasm_language_server::core","Core functionality related to documents.",null,null],[3,"Document","wasm_language_server::core::document","The current state of a document.",null,null],[12,"language","","The language type of the document, e.g., \\\"wasm.wast\\\"",0,null],[12,"parser","","The tree-sitter parser state for the document.",0,null],[12,"text","","The current text of the document.",0,null],[12,"tree","","The current tree-sitter parse tree of the document.",0,null],[11,"new","","Create a new Document for the given `language_id` and…",0,[[["string",3]],[["result",6],["option",4]]]],[0,"session","wasm_language_server::core","Core functionality related to the LSP server session.",null,null],[3,"Session","wasm_language_server::core::session","Represents the current state of the LSP service.",null,null],[11,"new","","Create a new session.",1,[[["option",4],["client",3]],["result",6]]],[11,"insert_document","","Insert an opened document into the session. Updates the…",1,[[["document",3],["url",3]],[["option",4],["result",6]]]],[11,"remove_document","","Remove a closed document from the session. Updates the…",1,[[["url",3]],[["option",4],["result",6]]]],[11,"get_document","","Get a reference to a document associated with the session,…",1,[[["url",3]]]],[11,"get_mut_document","","Get a mutable reference to a document associated with the…",1,[[["url",3]]]],[0,"lsp","wasm_language_server","Functionality related to implementation of the Language…",null,null],[0,"server","wasm_language_server::lsp","Definitions for the server instance.",null,null],[3,"Server","wasm_language_server::lsp::server","The WASM language server instance.",null,null],[5,"capabilities","","Compute the server capabilities.",null,[[],["servercapabilities",3]]],[11,"new","","Create a new server.",2,[[["client",3]],["result",6]]],[0,"provider","wasm_language_server","Providers of the WebAssembly language server for LSP…",null,null],[5,"document_symbol","wasm_language_server::provider","Provide response for `textDocument/documentSymbols`.",null,[[["arc",3],["documentsymbolparams",3],["session",3]]]],[0,"document_symbol","","Elaborates parse trees into structured data to be cached…",null,null],[0,"wast","wasm_language_server::provider::document_symbol","Elaborator definitions specific to \\\".wat\\\" and \\\".wast\\\"…",null,null],[5,"response","wasm_language_server::provider::document_symbol::wast","Compute \\\"textDocument/documentSymbols\\\" for a given document.",null,[[["document",3]]]],[0,"hover","wasm_language_server::provider","Provides `textDocument/hover` functionality.",null,null],[5,"response","wasm_language_server::provider::hover","Compute \\\"textDocument/hover\\\" for a given document.",null,[[["hoverparams",3],["document",3]]]],[0,"service","wasm_language_server","Services (components) of the WebAssembly language server.",null,null],[11,"from","wasm_language_server::core::document","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"init","","",0,[[]]],[11,"deref","","",0,[[]]],[11,"deref_mut","","",0,[[]]],[11,"drop","","",0,[[]]],[11,"from","wasm_language_server::core::session","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"init","","",1,[[]]],[11,"deref","","",1,[[]]],[11,"deref_mut","","",1,[[]]],[11,"drop","","",1,[[]]],[11,"from","wasm_language_server::lsp::server","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"init","","",2,[[]]],[11,"deref","","",2,[[]]],[11,"deref_mut","","",2,[[]]],[11,"drop","","",2,[[]]],[11,"initialize","","",2,[[["initializeparams",3]],[["pin",3],["box",3]]]],[11,"initialized","","",2,[[["initializedparams",3]],[["pin",3],["box",3]]]],[11,"shutdown","","",2,[[],[["pin",3],["box",3]]]],[11,"did_open","","",2,[[["didopentextdocumentparams",3]],[["pin",3],["box",3]]]],[11,"did_change","","",2,[[["didchangetextdocumentparams",3]],[["pin",3],["box",3]]]],[11,"did_close","","",2,[[["didclosetextdocumentparams",3]],[["pin",3],["box",3]]]],[11,"document_symbol","","",2,[[["documentsymbolparams",3]],[["pin",3],["box",3]]]],[11,"hover","","",2,[[["hoverparams",3]],[["box",3],["pin",3]]]],[11,"semantic_tokens_full","","",2,[[["semantictokensparams",3]],[["box",3],["pin",3]]]]],"p":[[3,"Document"],[3,"Session"],[3,"Server"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);