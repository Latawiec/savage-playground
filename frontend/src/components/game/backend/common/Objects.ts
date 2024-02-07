
export const hasValues = (obj: object) => Object.values(obj).some(v => v !== null && typeof v !== 'undefined')
