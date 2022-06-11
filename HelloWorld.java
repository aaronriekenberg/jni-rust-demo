interface MapGetter {
    void read(java.nio.ByteBuffer buffer);
}

class HelloWorld {

    private static native long newMap();

    private static native void deleteMap(long map);

    private static native long mapSize(long map);

    // private static native java.nio.ByteBuffer allocateBuffer(int size);

    // private static native void freeBuffer(java.nio.ByteBuffer buffer);

    private static native void putIntoMap(long map, byte[] key, byte[] value);

    private static native void getFromMap(long map, byte[] key, MapGetter getter);

    static {
        System.loadLibrary("mylib");
    }

    public static void main(String[] args) {

        final long map = newMap();
        System.out.println("map = " + map);

        // final java.nio.ByteBuffer byteBuffer = allocateBuffer(2048);

        // System.out.println("byteBuffer.isDirect = " + byteBuffer.isDirect());
        // System.out.println("byteBuffer.capacity = " + byteBuffer.capacity());

        // freeBuffer(byteBuffer);

        System.out.println("map size before put = " + mapSize(map));

        final byte[] key = { 0, 1, 2, 3};

        final byte[] value = {1,2,3,4,5};

        putIntoMap(map, key, value);

        System.out.println("map size after put = " + mapSize(map));

        System.out.println("before get(key)");

        getFromMap(map, key, (java.nio.ByteBuffer buffer) -> {
                System.out.println("in read, buffer.isDirect() = " + buffer.isDirect());
                System.out.println("in read, buffer.getCapacity() = " + buffer.capacity());
                System.out.println("in read, buffer.limit() = " + buffer.limit());

                for (int i = 0; i < buffer.limit(); ++i){
                    System.out.println("get = " + buffer.get());
                }
            }
        );

        System.out.println("before get(key)");

        getFromMap(map, key, (java.nio.ByteBuffer buffer) -> {
                System.out.println("in read, buffer.isDirect() = " + buffer.isDirect());
                System.out.println("in read, buffer.getCapacity() = " + buffer.capacity());
                System.out.println("in read, buffer.limit() = " + buffer.limit());

                for (int i = 0; i < buffer.limit(); ++i){
                    System.out.println("get = " + buffer.get());
                }
            }
        );

        System.out.println("before get(key2)");

        final byte[] key2 = { 0, 0, 0, 0 };
        getFromMap(map, key2, (java.nio.ByteBuffer buffer) -> {
                System.out.println("in read, buffer.isDirect() = " + buffer.isDirect());
                System.out.println("in read, buffer.getCapacity() = " + buffer.capacity());
                System.out.println("in read, buffer.limit() = " + buffer.limit());

                for (int i = 0; i < buffer.limit(); ++i){
                    System.out.println("get = " + buffer.get());
                }
            }
        );


        deleteMap(map);
    }

}
