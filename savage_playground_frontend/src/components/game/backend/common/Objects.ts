
export const hasValues = (obj: any) => Object.values(obj).some(v => v !== null && typeof v !== "undefined");