// @ts-check
/** @typedef {{id:string, token:string, minimum:number, expires:number}} Session */
/** @typedef {'unpaired'|'pairing'|'ready'|'choosing'|'sharing'|'pausing'|'paused'} Phase */
/** @typedef {{phase:Phase, session:Session|null, stream:MediaStream|null,
 * request:AbortController|null, timer:number|undefined, epoch:number,
 * interval:number, sent:number, status:string, resumeAllowed:boolean}} State */
/** @typedef {{state:State, video:HTMLVideoElement, refresh:()=>void,
 * halt:(unpair:boolean, notice:string, keepalive?:boolean)=>Promise<void>,
 * tick:(epoch:number)=>Promise<void>}} Context */
/** Construct volatile state: nothing is restored after a refresh.
 * @returns {State} */
export function initialState() {
  return {phase:'unpaired', session:null, stream:null, request:null,
    timer:undefined, epoch:0, interval:30, sent:0, resumeAllowed:true,
    status:'Open Screen in the iPhone app to get a pairing code.'};
}
/** Invalidate asynchronous work before releasing every capture track.
 * @param {Context} context */
export function release({state, video}) {
  state.epoch += 1;
  state.request?.abort(); state.request = null;
  clearTimeout(state.timer); state.timer = undefined;
  state.stream?.getTracks().forEach(track => track.stop());
  state.stream = null; video.srcObject = null;
}