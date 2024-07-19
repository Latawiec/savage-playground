
export interface RenderTarget {
    bind(): void;
    resize(width: number, height: number): void;
    
    get width(): number;
    get height(): number;
}
