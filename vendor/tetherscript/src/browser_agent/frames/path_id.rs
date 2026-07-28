use super::FrameId;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub(super) fn frame_id_for_path(path: &[usize]) -> FrameId {
    let mut hash = FNV_OFFSET;
    for index in path {
        hash ^= (*index as u64).wrapping_add(1);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    FrameId::new(hash.max(2))
}
