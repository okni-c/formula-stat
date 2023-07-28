import { motion } from "framer-motion";

export const LoadingScreen = () => {
    return (
        <motion.div className="bg-transparent w-full min-h-screen flex justify-center items-center"
            initial={false}
            animate={{  opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.5 }}>
            <div className="flex min-h-screen w-full items-center justify-center bg-transparent">
                <div className="flex w-48 items-center justify-center rounded-full bg-transparent animate-[spin_0.6s_linear_infinite]">
                    <img src="loading-tire.png" className="w-full animate-glow rounded-full" />
                </div>
            </div>
        </motion.div>
    );
};