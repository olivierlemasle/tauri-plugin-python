export declare function addResourcePathToSysPath(path: string): Promise<void>;
export declare function importModule(moduleName: string): Promise<void>;
export declare function callFunction(moduleName: string, functionName: string, args: Array<any>): Promise<any>;
