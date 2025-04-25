

struct kvPair<K, V> {
    K key;
    V value;
};

struct Vec<T> {
    u64 len;
    T data[len];
};

struct HashMap<K, V> {
    u64 size;
    kvPair<K, V> pairs[size];
};

using String = Vec<char>;

struct Tokenizer {
    HashMap<String, u64>;
    Vec<u64> id_to_token;
};

struct Model {
    u64 context_size;
    HashMap<Vec<u64>, HashMap<u64, u64>> context_to_next;
    Tokenizer tokenizer;
};

Model model @ 0x00;