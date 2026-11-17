import type { Snapshot } from './contract';
export function presentation(snapshot: Snapshot): { status: string; recording: string; startDisabled: boolean; advanceDisabled: boolean; pauseDisabled: boolean };
export function strategyFailure(failure: unknown, saving?: boolean): string;
export function sourceFailure(failure: unknown): string;
export function sourcePresentation(snapshot: import('./contract').SourceSnapshot, name: import('./contract').SourceName): string;
