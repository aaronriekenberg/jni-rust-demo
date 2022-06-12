class HelloWorld {

    private final RustMap rustMap = new RustMap();

    public void run() {

        System.out.println("rustMap size before put = " + rustMap.size());

        final byte[] key = { 0, 1, 2, 3};

        final byte[] value = {1,2,3,4,5};

        rustMap.put(key, value);

        System.out.println("map size after put = " + rustMap.size());

        System.out.println("before get(key)");

        long startNano = System.nanoTime();

        rustMap.get(key, (java.nio.ByteBuffer buffer) -> {
                System.out.println("in read, buffer.isDirect() = " + buffer.isDirect());
                System.out.println("in read, buffer.getCapacity() = " + buffer.capacity());
                System.out.println("in read, buffer.limit() = " + buffer.limit());

                for (int i = 0; i < buffer.limit(); ++i){
                    System.out.println("get = " + buffer.get());
                }
            }
        );
    
        long deltaNano = System.nanoTime() - startNano;

        System.out.println("deltaNano = " + deltaNano);

        System.out.println("before get(key)");

        rustMap.get(key, (java.nio.ByteBuffer buffer) -> {
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
        rustMap.get(key2, (java.nio.ByteBuffer buffer) -> {
                System.out.println("in read, buffer.isDirect() = " + buffer.isDirect());
                System.out.println("in read, buffer.getCapacity() = " + buffer.capacity());
                System.out.println("in read, buffer.limit() = " + buffer.limit());

                for (int i = 0; i < buffer.limit(); ++i){
                    System.out.println("get = " + buffer.get());
                }
            }
        );

        rustMap.delete(key);

        System.out.println("map size after delete = " + rustMap.size());

        rustMap.destroy();
    }
 
    public static void main(String[] args) {
        new HelloWorld().run();
    }

}
