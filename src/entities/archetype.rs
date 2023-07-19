type ArchetypeId = u32;
struct Storage<T> {
    slice: HashMap<ArchetypeId, [T]>
}

type ArchWorld<T> = HashMap<String, Storage<T>>;