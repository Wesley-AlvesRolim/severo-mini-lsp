import path from "node:path";
import * as vscode from "vscode";
import {
	type LanguageClientOptions,
	RevealOutputChannelOn,
} from "vscode-languageclient";
import {
	type ServerOptions,
	TransportKind,
} from "vscode-languageclient/node.js";

export const clientId = "severo-vscode-lsclient";
export const clientName = "Severo LS Client";
export const clientOptions = (
	outputChannel: LanguageClientOptions["outputChannel"],
): LanguageClientOptions => ({
	documentSelector: [{ scheme: "file", language: "severo" }],
	outputChannel,
	revealOutputChannelOn: RevealOutputChannelOn.Never,
	synchronize: {
		fileEvents: vscode.workspace.createFileSystemWatcher("**/.clientrc"),
	},
});

export const LS_LAUNCHER_MAIN: string = "SeveroLanguageServerLauncher";
export function getServerOptions(): ServerOptions {
	const executable: string = path.resolve(
		__dirname,
		"..",
		"..",
		"target",
		"release",
		"severo-lsp",
	);
	return {
		run: {
			command: executable,
			transport: TransportKind.stdio,
		},
		debug: {
			command: executable,
			transport: TransportKind.stdio,
		},
	};
}
