export function form(prompt, at) {
  const inner = Array(69).fill(null);
  inner[0] = [prompt, 0, null, null, null, null, 0];
  inner[1] = ["en"];
  inner[2] = ["", "", "", null, null, null, null, null, null, ""];
  inner[6] = [1]; inner[7] = 1; inner[10] = 1; inner[11] = 0;
  inner[17] = [[0]]; inner[18] = 0; inner[27] = 1; inner[30] = [4];
  inner[53] = 0; inner[59] = "CD1035A5-0E0E-4B68-B744-23C2D8960DF5";
  inner[61] = []; inner[66] = [Math.floor(Date.now() / 1000), 0]; inner[68] = 2;
  return new URLSearchParams({ "f.req": JSON.stringify([null, JSON.stringify(inner)]), at });
}

export function modelHeader(mode = "5bf011840784117a") {
  return JSON.stringify([1, null, null, null, mode, null, null, 0, [4], null, null, 3]);
}
