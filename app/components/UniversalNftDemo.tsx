import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PublicKey, Keypair, Transaction } from '@solana/web3.js';
import { toast } from 'react-hot-toast';
import { Zap, Shield, Globe, Cpu, Award, TrendingUp } from 'lucide-react';

interface NFTDemoState {
  step: number;
  nftData: {
    name: string;
    symbol: string;
    uri: string;
    metadata: any;
  };
  transactionStatus: 'idle' | 'minting' | 'transferring' | 'completed' | 'error';
  mintedNft: any;
  crossChainTransfer: any;
}

// Mock NFT Metadata Generator
const generateNFTMetadata = (name: string) => ({
  name,
  symbol: name.split(' ').map(w => w[0]).join('').toUpperCase(),
  description: `A revolutionary Universal NFT that exists across multiple blockchains via ZetaChain protocol`,
  image: `https://api.dicebear.com/7.x/shapes/svg?seed=${name}&backgroundColor=8b5cf6,06b6d4,10b981`,
  attributes: [
    { trait_type: "Rarity", value: "Legendary" },
    { trait_type: "Cross-Chain", value: "Enabled" },
    { trait_type: "Security Score", value: "99.9%" },
    { trait_type: "Generation", value: "Universal V1" },
    { trait_type: "Timestamp", value: new Date().toISOString() }
  ],
  properties: {
    category: "universal-nft",
    creators: [
      {
        address: "ZetaChainUniversalNFTProgram",
        share: 100
      }
    ]
  }
});

// NFT Preview Card Component
const NFTPreviewCard = ({ nft, onSelect }: any) => {
  const [hovered, setHovered] = useState(false);
  
  return (
    <motion.div
      className="relative p-6 bg-gradient-to-br from-purple-900/50 to-blue-900/50 rounded-xl border border-purple-500/30 cursor-pointer"
      whileHover={{ scale: 1.02, y: -5 }}
      whileTap={{ scale: 0.98 }}
      onHoverStart={() => setHovered(true)}
      onHoverEnd={() => setHovered(false)}
      onClick={() => onSelect(nft)}
    >
      <div className="aspect-square bg-gradient-to-br from-purple-600 to-pink-600 rounded-lg mb-4 flex items-center justify-center overflow-hidden">
        <img 
          src={nft.image} 
          alt={nft.name}
          className="w-full h-full object-cover"
        />
      </div>
      
      <h3 className="text-xl font-bold text-white mb-2">{nft.name}</h3>
      <p className="text-gray-300 text-sm mb-4">{nft.description}</p>
      
      <div className="flex flex-wrap gap-2 mb-4">
        {nft.attributes.slice(0, 2).map((attr: any, idx: number) => (
          <span 
            key={idx}
            className="px-2 py-1 bg-purple-600/50 text-purple-200 text-xs rounded-full"
          >
            {attr.trait_type}: {attr.value}
          </span>
        ))}
      </div>
      
      <motion.div
        className="flex items-center justify-between"
        animate={{ opacity: hovered ? 1 : 0.7 }}
      >
        <span className="text-purple-400 font-bold">Universal NFT</span>
        <Zap className="w-4 h-4 text-yellow-400" />
      </motion.div>
      
      {hovered && (
        <motion.div
          className="absolute inset-0 bg-purple-600/10 rounded-xl border-2 border-purple-400"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.2 }}
        />
      )}
    </motion.div>
  );
};

// Step Progress Indicator
const StepProgress = ({ currentStep, totalSteps }: { currentStep: number; totalSteps: number }) => {
  return (
    <div className="flex items-center justify-center mb-12">
      {Array.from({ length: totalSteps }, (_, i) => (
        <React.Fragment key={i}>
          <motion.div
            className={`w-10 h-10 rounded-full flex items-center justify-center border-2 ${
              i < currentStep
                ? 'bg-green-500 border-green-500 text-white'
                : i === currentStep
                ? 'bg-purple-600 border-purple-600 text-white'
                : 'bg-gray-700 border-gray-600 text-gray-400'
            }`}
            initial={{ scale: 0.8 }}
            animate={{ scale: i === currentStep ? 1.1 : 1 }}
            transition={{ duration: 0.3 }}
          >
            {i < currentStep ? '✓' : i + 1}
          </motion.div>
          {i < totalSteps - 1 && (
            <motion.div
              className={`w-16 h-1 mx-2 ${
                i < currentStep ? 'bg-green-500' : 'bg-gray-600'
              }`}
              initial={{ scaleX: 0 }}
              animate={{ scaleX: i < currentStep ? 1 : 0 }}
              transition={{ duration: 0.5, delay: i * 0.1 }}
            />
          )}
        </React.Fragment>
      ))}
    </div>
  );
};

// Cross-Chain Transfer Visualizer
const CrossChainVisualizer = ({ transfer }: any) => {
  const chains = [
    { name: 'Solana', color: '#9945FF', position: 0 },
    { name: 'ZetaChain', color: '#00D4FF', position: 50 },
    { name: 'Ethereum', color: '#627EEA', position: 100 },
  ];
  
  const [progress, setProgress] = useState(0);
  
  useEffect(() => {
    if (transfer?.status === 'transferring') {
      const interval = setInterval(() => {
        setProgress(prev => {
          if (prev >= 100) {
            clearInterval(interval);
            return 100;
          }
          return prev + 2;
        });
      }, 100);
      return () => clearInterval(interval);
    }
  }, [transfer?.status]);
  
  return (
    <div className="relative py-8">
      <div className="flex justify-between items-center mb-8">
        {chains.map((chain, idx) => (
          <motion.div
            key={chain.name}
            className="flex flex-col items-center"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: idx * 0.2 }}
          >
            <div 
              className="w-16 h-16 rounded-full flex items-center justify-center text-white font-bold text-lg shadow-lg"
              style={{ backgroundColor: chain.color }}
            >
              {chain.name[0]}
            </div>
            <span className="mt-2 text-sm text-gray-300">{chain.name}</span>
          </motion.div>
        ))}
      </div>
      
      <div className="relative h-2 bg-gray-700 rounded-full overflow-hidden">
        <motion.div
          className="h-full bg-gradient-to-r from-purple-500 to-pink-500 rounded-full"
          initial={{ width: '0%' }}
          animate={{ width: `${progress}%` }}
          transition={{ duration: 0.3 }}
        />
        
        {progress > 0 && (
          <motion.div
            className="absolute top-1/2 transform -translate-y-1/2 w-4 h-4 bg-yellow-400 rounded-full shadow-lg"
            style={{ left: `${progress}%` }}
            animate={{ 
              boxShadow: ['0 0 0px rgba(255,255,0,0.5)', '0 0 20px rgba(255,255,0,0.8)', '0 0 0px rgba(255,255,0,0.5)']
            }}
            transition={{ duration: 1, repeat: Infinity }}
          />
        )}
      </div>
      
      <div className="mt-4 text-center">
        <span className="text-purple-400 font-bold">
          {progress === 0 ? 'Ready to Transfer' : 
           progress < 100 ? `Transferring... ${progress}%` : 
           'Transfer Complete!'}
        </span>
      </div>
    </div>
  );
};

// Main Demo Component
export default function UniversalNftDemo() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const [demoState, setDemoState] = useState<NFTDemoState>({
    step: 0,
    nftData: {
      name: '',
      symbol: '',
      uri: '',
      metadata: null,
    },
    transactionStatus: 'idle',
    mintedNft: null,
    crossChainTransfer: null,
  });

  const prebuiltNFTs = [
    generateNFTMetadata("Cosmic Warrior"),
    generateNFTMetadata("Digital Phoenix"),
    generateNFTMetadata("Quantum Crystal"),
    generateNFTMetadata("Cyber Dragon"),
  ];

  const handleNFTSelection = (selectedNft: any) => {
    setDemoState(prev => ({
      ...prev,
      nftData: {
        name: selectedNft.name,
        symbol: selectedNft.symbol,
        uri: `https://arweave.net/${Math.random().toString(36).substring(7)}`,
        metadata: selectedNft,
      },
      step: 1,
    }));
    toast.success(`Selected ${selectedNft.name} for minting!`);
  };

  const handleMintNFT = async () => {
    if (!wallet.connected) {
      toast.error('Please connect your wallet first!');
      return;
    }

    setDemoState(prev => ({ ...prev, transactionStatus: 'minting', step: 2 }));
    
    try {
      // Simulate minting process
      await new Promise(resolve => setTimeout(resolve, 3000));
      
      const mockMintedNft = {
        mint: Keypair.generate().publicKey.toString(),
        owner: wallet.publicKey?.toString(),
        tokenId: `universal_${Date.now()}`,
        ...demoState.nftData,
      };
      
      setDemoState(prev => ({
        ...prev,
        transactionStatus: 'completed',
        mintedNft: mockMintedNft,
        step: 3,
      }));
      
      toast.success('🎉 Universal NFT minted successfully!');
    } catch (error) {
      setDemoState(prev => ({ ...prev, transactionStatus: 'error' }));
      toast.error('Minting failed. Please try again.');
    }
  };

  const handleCrossChainTransfer = async (targetChain: string) => {
    setDemoState(prev => ({
      ...prev,
      transactionStatus: 'transferring',
      crossChainTransfer: {
        from: 'Solana',
        to: targetChain,
        status: 'transferring',
      },
      step: 4,
    }));

    try {
      // Simulate cross-chain transfer
      await new Promise(resolve => setTimeout(resolve, 5000));
      
      setDemoState(prev => ({
        ...prev,
        transactionStatus: 'completed',
        crossChainTransfer: {
          ...prev.crossChainTransfer,
          status: 'completed',
        },
      }));
      
      toast.success(`🌉 NFT successfully transferred to ${targetChain}!`);
    } catch (error) {
      setDemoState(prev => ({ ...prev, transactionStatus: 'error' }));
      toast.error('Transfer failed. Please try again.');
    }
  };

  const renderStep = () => {
    switch (demoState.step) {
      case 0:
        return (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
          >
            <h3 className="text-3xl font-bold text-white text-center mb-8">
              Choose Your Universal NFT
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
              {prebuiltNFTs.map((nft, idx) => (
                <NFTPreviewCard
                  key={idx}
                  nft={nft}
                  onSelect={handleNFTSelection}
                />
              ))}
            </div>
          </motion.div>
        );

      case 1:
        return (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            className="text-center"
          >
            <h3 className="text-3xl font-bold text-white mb-8">
              Connect Wallet & Mint
            </h3>
            
            <div className="bg-gray-800 rounded-xl p-8 mb-8 max-w-md mx-auto">
              <img 
                src={demoState.nftData.metadata?.image} 
                alt={demoState.nftData.name}
                className="w-full aspect-square rounded-lg mb-4"
              />
              <h4 className="text-xl font-bold text-white mb-2">
                {demoState.nftData.name}
              </h4>
              <p className="text-gray-300 mb-4">
                {demoState.nftData.metadata?.description}
              </p>
            </div>

            <div className="space-y-4">
              <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
              
              {wallet.connected && (
                <motion.button
                  className="px-8 py-4 bg-gradient-to-r from-green-600 to-blue-600 text-white font-bold text-lg rounded-full shadow-lg hover:shadow-2xl transform transition-all duration-300"
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  onClick={handleMintNFT}
                  disabled={demoState.transactionStatus === 'minting'}
                >
                  {demoState.transactionStatus === 'minting' ? '⏳ Minting...' : '🎨 Mint Universal NFT'}
                </motion.button>
              )}
            </div>
          </motion.div>
        );

      case 2:
        return (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            className="text-center"
          >
            <h3 className="text-3xl font-bold text-white mb-8">
              Minting Your Universal NFT
            </h3>
            
            <div className="relative">
              <motion.div
                className="w-32 h-32 mx-auto mb-8 rounded-full bg-gradient-to-r from-purple-600 to-pink-600 flex items-center justify-center"
                animate={{ rotate: 360 }}
                transition={{ duration: 2, repeat: Infinity, ease: "linear" }}
              >
                <Zap className="w-16 h-16 text-white" />
              </motion.div>
              
              <div className="space-y-4 text-gray-300">
                <motion.div
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 0.5 }}
                  className="flex items-center justify-center gap-2"
                >
                  <Shield className="w-5 h-5 text-green-400" />
                  <span>Security verification complete</span>
                </motion.div>
                
                <motion.div
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 1 }}
                  className="flex items-center justify-center gap-2"
                >
                  <Cpu className="w-5 h-5 text-blue-400" />
                  <span>Optimizing compute units</span>
                </motion.div>
                
                <motion.div
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 1.5 }}
                  className="flex items-center justify-center gap-2"
                >
                  <Globe className="w-5 h-5 text-purple-400" />
                  <span>Enabling cross-chain compatibility</span>
                </motion.div>
              </div>
            </div>
          </motion.div>
        );

      case 3:
        return (
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.6 }}
            className="text-center"
          >
            <motion.div
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={{ delay: 0.3, type: "spring", stiffness: 200 }}
            >
              <Award className="w-24 h-24 text-yellow-400 mx-auto mb-6" />
            </motion.div>
            
            <h3 className="text-4xl font-bold text-white mb-4">
              🎉 NFT Minted Successfully!
            </h3>
            
            <div className="bg-gray-800 rounded-xl p-6 max-w-md mx-auto mb-8">
              <img 
                src={demoState.nftData.metadata?.image} 
                alt={demoState.nftData.name}
                className="w-full aspect-square rounded-lg mb-4"
              />
              <h4 className="text-xl font-bold text-white mb-2">
                {demoState.nftData.name}
              </h4>
              <p className="text-green-400 font-bold mb-2">
                Token ID: {demoState.mintedNft?.tokenId}
              </p>
              <p className="text-gray-300 text-sm">
                Your NFT is now live on Solana and ready for cross-chain adventures!
              </p>
            </div>

            <h4 className="text-2xl font-bold text-white mb-6">
              Transfer to Another Chain
            </h4>
            
            <div className="flex flex-wrap gap-4 justify-center">
              {['Ethereum', 'BSC', 'Polygon'].map(chain => (
                <motion.button
                  key={chain}
                  className="px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold rounded-full shadow-lg hover:shadow-2xl transform transition-all duration-300"
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  onClick={() => handleCrossChainTransfer(chain)}
                >
                  🌉 Transfer to {chain}
                </motion.button>
              ))}
            </div>
          </motion.div>
        );

      case 4:
        return (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            className="text-center"
          >
            <h3 className="text-3xl font-bold text-white mb-8">
              Cross-Chain Transfer in Progress
            </h3>
            
            <CrossChainVisualizer transfer={demoState.crossChainTransfer} />
            
            <div className="mt-8 space-y-4 text-gray-300">
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.5 }}
                className="flex items-center justify-center gap-2"
              >
                <TrendingUp className="w-5 h-5 text-blue-400" />
                <span>Burning NFT on Solana...</span>
              </motion.div>
              
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 1.5 }}
                className="flex items-center justify-center gap-2"
              >
                <Globe className="w-5 h-5 text-purple-400" />
                <span>Routing through ZetaChain...</span>
              </motion.div>
              
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 2.5 }}
                className="flex items-center justify-center gap-2"
              >
                <Zap className="w-5 h-5 text-green-400" />
                <span>Minting on destination chain...</span>
              </motion.div>
            </div>
            
            {demoState.crossChainTransfer?.status === 'completed' && (
              <motion.div
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ delay: 3, type: "spring" }}
                className="mt-8 p-6 bg-green-900/50 border border-green-500/50 rounded-xl"
              >
                <h4 className="text-2xl font-bold text-green-400 mb-2">
                  🎉 Transfer Complete!
                </h4>
                <p className="text-green-200">
                  Your Universal NFT is now live on {demoState.crossChainTransfer.to}!
                </p>
              </motion.div>
            )}
          </motion.div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="max-w-6xl mx-auto">
      <StepProgress currentStep={demoState.step} totalSteps={5} />
      
      <AnimatePresence mode="wait">
        <motion.div
          key={demoState.step}
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: -20 }}
          transition={{ duration: 0.5 }}
        >
          {renderStep()}
        </motion.div>
      </AnimatePresence>
      
      {demoState.step > 0 && (
        <motion.div
          className="flex justify-center mt-12"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 1 }}
        >
          <button
            className="px-6 py-3 bg-gray-700 text-gray-300 rounded-full hover:bg-gray-600 transition-colors"
            onClick={() => setDemoState({
              step: 0,
              nftData: { name: '', symbol: '', uri: '', metadata: null },
              transactionStatus: 'idle',
              mintedNft: null,
              crossChainTransfer: null,
            })}
          >
            🔄 Start Over
          </button>
        </motion.div>
      )}
    </div>
  );
}