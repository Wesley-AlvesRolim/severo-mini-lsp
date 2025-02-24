import * as vscode from "vscode";
import { extensionInstance } from "./core/extension.js";

export function activate(context: vscode.ExtensionContext) {
	//Set the context of the extension instance
	extensionInstance.setContext(context);
	//Initialize the LS Client extension instance.
	extensionInstance.init().catch((error: Error) => {
		console.log("Failed to activate Severo extension. " + error);
	});
}

export function deactivate() {}
