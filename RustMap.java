interface MapGetter {
    void read(java.nio.ByteBuffer buffer);
}

class RustMap {

   static {
        System.loadLibrary("mylib");
    }

    private static native long newMap();

    private static native void deleteMap(long map);

    private static native long mapSize(long map);

    private static native void putIntoMap(long map, byte[] key, byte[] value);

    private static native void getFromMap(long map, byte[] key, MapGetter getter);

    private static native void deleteFromMap(long map, byte[] key);

    private long map;

    public RustMap() {
        this.map = newMap();
    }

    public void destroy() {
        if (this.map != 0) {
            deleteMap(this.map);
            this.map = 0;
        }
    }

    public void put(byte[] key, byte[] value) {
        putIntoMap(this.map, key, value);
    }

    public void get(byte[] key, MapGetter getter) {
        getFromMap(this.map, key, getter);
    }

    public long size() {
        return mapSize(this.map);
    }

    public void delete(byte[] key) {
        deleteFromMap(this.map, key);
    }

}
