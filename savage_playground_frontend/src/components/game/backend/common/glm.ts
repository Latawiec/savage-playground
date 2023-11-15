import { mat4 } from "gl-matrix";

export namespace glm {

export function mat4_from_array(values: number[]): mat4 {
    if (values.length != 16) {
        console.error(`Trying to make mat4 from array of size ${values.length}. It should be 16. Returning identity.`);
        return mat4.create();
    }
    return mat4.fromValues(
        values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8],
        values[9], values[10], values[11], values[12], values[13], values[14], values[15]
    );
}

}