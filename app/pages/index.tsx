import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Text, Box, Sphere } from '@react-three/drei';
import { WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
import { ConnectionProvider } from '@solana/wallet-adapter-react';
import { Toaster } from 'react-hot-toast';
import confetti from 'canvas-confetti';

import UniversalNftDemo from '../components/UniversalNftDemo';
import CrossChainVisualizer from '../components/CrossChainVisualizer';
import SecurityDashboard from '../components/SecurityDashboard';
import PerformanceMetrics from '../components/PerformanceMetrics';
import InteractiveDocumentation from '../components/InteractiveDocumentation';

require('@solana/wallet-adapter-react-ui/styles.css');

// 3D NFT Visualization Component
function FloatingNFT({ position, rotation, scale }: any) {
  return (
    <motion.group
      position={position}
      rotation={rotation}
      scale={scale}
      animate={{
        rotateY: [0, Math.PI * 2],
        y: [position[1] - 0.5, position[1] + 0.5, position[1] - 0.5],
      }}
      transition={{
        rotateY: { duration: 8, repeat: Infinity, ease: "linear" },
        y: { duration: 4, repeat: Infinity, ease: "easeInOut" },
      }}
    >
      <Box args={[1, 1, 0.1]}>
        <meshStandardMaterial color="#8B5CF6" metalness={0.8} roughness={0.2} />
      </Box>
      <Text
        position={[0, 0, 0.06]}
        fontSize={0.2}
        color="white"
        anchorX="center"
        anchorY="middle"
        font="/fonts/roboto-regular.woff"
      >
        Universal
        {'\n'}
        NFT
      </Text>
    </motion.group>
  );
}

// Blockchain Network Nodes
function NetworkNode({ position, color, label }: any) {
  const [hovered, setHovered] = useState(false);
  
  return (
    <group position={position}>
      <Sphere
        args={[0.5]}
        onPointerOver={() => setHovered(true)}
        onPointerOut={() => setHovered(false)}
        scale={hovered ? 1.2 : 1}
      >
        <meshStandardMaterial 
          color={color} 
          emissive={color} 
          emissiveIntensity={hovered ? 0.3 : 0.1}
        />
      </Sphere>
      <Text
        position={[0, -0.8, 0]}
        fontSize={0.3}
        color="white"
        anchorX="center"
        anchorY="middle"
      >
        {label}
      </Text>
    </group>
  );
}

// 3D Cross-Chain Bridge Visualization
function CrossChainBridge() {
  return (
    <Canvas className="w-full h-96">
      <ambientLight intensity={0.5} />
      <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} />
      <pointLight position={[-10, -10, -10]} />
      
      {/* Blockchain Networks */}
      <NetworkNode position={[-4, 0, 0]} color="#9945FF" label="Solana" />
      <NetworkNode position={[0, 2, 0]} color="#00D4FF" label="ZetaChain" />
      <NetworkNode position={[4, 0, 0]} color="#627EEA" label="Ethereum" />
      <NetworkNode position={[0, -2, 0]} color="#F3BA2F" label="BSC" />
      
      {/* Floating NFTs */}
      <FloatingNFT position={[-2, 1, 0]} rotation={[0, 0, 0]} scale={0.5} />
      <FloatingNFT position={[2, -1, 0]} rotation={[0, Math.PI, 0]} scale={0.5} />
      
      {/* Connection Lines */}
      <mesh>
        <cylinderGeometry args={[0.02, 0.02, 4.47]} />
        <meshStandardMaterial color="#8B5CF6" emissive="#8B5CF6" emissiveIntensity={0.2} />
      </mesh>
      
      <OrbitControls enableZoom={true} enablePan={true} enableRotate={true} />
    </Canvas>
  );
}

// Hero Section with Advanced Animations
function HeroSection() {
  const [currentFeature, setCurrentFeature] = useState(0);
  const features = [
    "Universal Cross-Chain NFTs",
    "Advanced Security System", 
    "Real-Time Fraud Detection",
    "Performance Optimization",
    "Enterprise-Grade Tooling"
  ];

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentFeature((prev) => (prev + 1) % features.length);
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  const triggerCelebration = () => {
    confetti({
      particleCount: 100,
      spread: 70,
      origin: { y: 0.6 }
    });
  };

  return (
    <section className="relative min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 overflow-hidden">
      {/* Animated Background */}
      <div className="absolute inset-0">
        <div className="absolute w-96 h-96 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob"></div>
        <div className="absolute w-96 h-96 bg-yellow-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-2000 top-0 right-0"></div>
        <div className="absolute w-96 h-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-4000 bottom-0 left-0"></div>
      </div>

      <div className="relative z-10 container mx-auto px-6 py-20">
        <div className="text-center">
          <motion.h1 
            className="text-6xl md:text-8xl font-bold text-white mb-8 leading-tight"
            initial={{ opacity: 0, y: 50 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 1 }}
          >
            ZetaChain
            <br />
            <span className="bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
              Universal NFT
            </span>
          </motion.h1>

          <motion.div
            className="text-2xl md:text-3xl text-gray-300 mb-12 h-20"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.5, duration: 1 }}
          >
            <motion.span
              key={currentFeature}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              transition={{ duration: 0.5 }}
              className="block"
            >
              {features[currentFeature]}
            </motion.span>
          </motion.div>

          <motion.div
            className="flex flex-col md:flex-row gap-6 justify-center items-center mb-16"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 1, duration: 1 }}
          >
            <motion.button
              className="px-8 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold text-lg rounded-full shadow-lg hover:shadow-2xl transform transition-all duration-300"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              onClick={triggerCelebration}
            >
              🚀 Launch Demo
            </motion.button>
            
            <motion.button
              className="px-8 py-4 border-2 border-purple-400 text-purple-400 font-bold text-lg rounded-full hover:bg-purple-400 hover:text-white transition-all duration-300"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              📚 Explore Docs
            </motion.button>
          </motion.div>

          {/* 3D Visualization */}
          <motion.div
            className="mb-16"
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 1.5, duration: 1 }}
          >
            <CrossChainBridge />
          </motion.div>

          {/* Stats Section */}
          <motion.div
            className="grid grid-cols-2 md:grid-cols-4 gap-8 text-white"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 2, duration: 1 }}
          >
            <div className="text-center">
              <div className="text-4xl font-bold text-purple-400">5+</div>
              <div className="text-gray-300">Supported Chains</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-purple-400">99.9%</div>
              <div className="text-gray-300">Security Score</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-purple-400">&lt;50k</div>
              <div className="text-gray-300">Compute Units</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-purple-400">∞</div>
              <div className="text-gray-300">Possibilities</div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  );
}

// Main App Component
export default function Home() {
  const endpoint = process.env.NEXT_PUBLIC_SOLANA_RPC_HOST!;
  const wallets = [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter(),
  ];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div className="min-h-screen bg-gray-900">
            <Toaster 
              position="top-right"
              toastOptions={{
                style: {
                  background: '#1F2937',
                  color: '#F9FAFB',
                  border: '1px solid #374151',
                },
              }}
            />
            
            <HeroSection />
            
            {/* Demo Sections */}
            <section className="py-20 bg-gray-800">
              <div className="container mx-auto px-6">
                <motion.h2 
                  className="text-5xl font-bold text-center text-white mb-16"
                  initial={{ opacity: 0, y: 30 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 1 }}
                >
                  🎮 Interactive Demo
                </motion.h2>
                <UniversalNftDemo />
              </div>
            </section>

            {/* Cross-Chain Visualizer */}
            <section className="py-20 bg-gray-900">
              <div className="container mx-auto px-6">
                <motion.h2 
                  className="text-5xl font-bold text-center text-white mb-16"
                  initial={{ opacity: 0, y: 30 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 1 }}
                >
                  🌉 Cross-Chain Visualizer
                </motion.h2>
                <CrossChainVisualizer />
              </div>
            </section>

            {/* Security Dashboard */}
            <section className="py-20 bg-gray-800">
              <div className="container mx-auto px-6">
                <motion.h2 
                  className="text-5xl font-bold text-center text-white mb-16"
                  initial={{ opacity: 0, y: 30 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 1 }}
                >
                  🛡️ Security Dashboard
                </motion.h2>
                <SecurityDashboard />
              </div>
            </section>

            {/* Performance Metrics */}
            <section className="py-20 bg-gray-900">
              <div className="container mx-auto px-6">
                <motion.h2 
                  className="text-5xl font-bold text-center text-white mb-16"
                  initial={{ opacity: 0, y: 30 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 1 }}
                >
                  ⚡ Performance Metrics
                </motion.h2>
                <PerformanceMetrics />
              </div>
            </section>

            {/* Interactive Documentation */}
            <section className="py-20 bg-gray-800">
              <div className="container mx-auto px-6">
                <motion.h2 
                  className="text-5xl font-bold text-center text-white mb-16"
                  initial={{ opacity: 0, y: 30 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 1 }}
                >
                  📚 Interactive Documentation
                </motion.h2>
                <InteractiveDocumentation />
              </div>
            </section>
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}