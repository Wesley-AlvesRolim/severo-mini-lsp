import * as vscode from "vscode";
import { LanguageClient, State } from "vscode-languageclient/node.js";
import {
	clientId,
	clientName,
	clientOptions,
	getServerOptions,
} from "./ls-configs.js";

const outputChannel = vscode.window.createOutputChannel("severo");

export class SeveroExtension {
	private languageClient?: LanguageClient;
	private context?: vscode.ExtensionContext;

	setContext(context: vscode.ExtensionContext) {
		this.context = context;
	}

	async init(): Promise<void> {
		try {
			this.languageClient = new LanguageClient(
				clientId,
				clientName,
				getServerOptions(),
				clientOptions(outputChannel),
			);

			this.languageClient.onDidChangeState((stateChangeEvent) => {
				if (stateChangeEvent.newState === State.Stopped) {
					vscode.window.showErrorMessage("Failed to initialize the extension");
				} else if (stateChangeEvent.newState === State.Running) {
					vscode.window.showInformationMessage(
						"Extension initialized successfully!",
					);
				}
			});

			this.languageClient.start();
		} catch (exception) {
			return Promise.reject("Extension error!");
		}
	}
}

export const extensionInstance = new SeveroExtension();
