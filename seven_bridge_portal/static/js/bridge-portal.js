// Seven Bridge Portal Interactive Features
// Inspired by Gwern.net's sophisticated interaction design

class SevenBridgePortal {
    constructor() {
        this.currentBridge = 1;
        this.crossReferences = new Map();
        this.verificationMatrix = new Map();
        
        this.init();
    }
    
    init() {
        this.setupBridgeNavigation();
        this.setupCrossReferences();
        this.setupSearchFunctionality();
        this.setupVerificationMatrix();
        this.setupMathRendering();
        this.setupReadingProgress();
    }
    
    setupBridgeNavigation() {
        const bridgeLinks = document.querySelectorAll('.bridge-link');
        
        bridgeLinks.forEach((link, index) => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                this.navigateToBridge(index + 1);
            });
            
            // Add keyboard navigation
            link.addEventListener('keydown', (e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    this.navigateToBridge(index + 1);
                }
            });
        });
        
        // Keyboard shortcuts for bridge navigation
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey || e.metaKey) {
                const num = parseInt(e.key);
                if (num >= 1 && num <= 7) {
                    e.preventDefault();
                    this.navigateToBridge(num);
                }
            }
        });
    }
    
    navigateToBridge(bridgeNumber) {
        // Remove active class from all bridges
        document.querySelectorAll('.bridge-link').forEach(link => {
            link.classList.remove('active');
        });
        
        // Add active class to selected bridge
        const targetBridge = document.querySelector(`[data-bridge-num="${bridgeNumber}"]`);
        if (targetBridge) {
            targetBridge.classList.add('active');
            this.currentBridge = bridgeNumber;
            
            // Smooth scroll to content
            const bookContent = document.querySelector(`#bridge-${bridgeNumber}`);
            if (bookContent) {
                bookContent.scrollIntoView({ 
                    behavior: 'smooth', 
                    block: 'start' 
                });
            }
            
            // Update URL without triggering navigation
            const newUrl = new URL(window.location);
            newUrl.searchParams.set('bridge', bridgeNumber);
            window.history.replaceState({}, '', newUrl);
            
            // Update reading progress
            this.updateReadingProgress();
        }
    }
    
    setupCrossReferences() {
        const crossRefElements = document.querySelectorAll('.cross-reference');
        
        crossRefElements.forEach(element => {
            element.addEventListener('mouseenter', (e) => {
                this.showCrossReferenceTooltip(e.target);
            });
            
            element.addEventListener('mouseleave', () => {
                this.hideCrossReferenceTooltip();
            });
            
            element.addEventListener('click', (e) => {
                e.preventDefault();
                this.navigateToCrossReference(e.target.getAttribute('data-ref'));
            });
        });
    }
    
    showCrossReferenceTooltip(element) {
        const ref = element.getAttribute('data-ref');
        const tooltip = document.createElement('div');
        tooltip.className = 'cross-ref-tooltip';
        tooltip.innerHTML = this.getCrossReferenceContent(ref);
        
        document.body.appendChild(tooltip);
        
        // Position tooltip
        const rect = element.getBoundingClientRect();
        tooltip.style.position = 'absolute';
        tooltip.style.left = `${rect.left}px`;
        tooltip.style.top = `${rect.bottom + 10}px`;
        tooltip.style.zIndex = '1000';
        tooltip.style.background = 'var(--mist-gray)';
        tooltip.style.border = '1px solid var(--stone-bridge-gray)';
        tooltip.style.borderRadius = '0.5rem';
        tooltip.style.padding = '1rem';
        tooltip.style.maxWidth = '300px';
        tooltip.style.fontSize = 'var(--text-sm)';
        tooltip.style.boxShadow = '0 4px 20px var(--bridge-shadow)';
    }
    
    hideCrossReferenceTooltip() {
        const tooltip = document.querySelector('.cross-ref-tooltip');
        if (tooltip) {
            tooltip.remove();
        }
    }
    
    getCrossReferenceContent(ref) {
        // This would fetch content from the cross-reference database
        const mockContent = {
            'bridge-01:recursive-architecture': 'Self-modifying systems that can analyze their own structure...',
            'bridge-02:strategic-loops': 'Business strategies that reference themselves create...',
            'bridge-03:pattern-recognition': 'AI systems that recognize patterns in their own pattern recognition...'
        };
        
        return mockContent[ref] || 'Cross-reference content loading...';
    }
    
    navigateToCrossReference(ref) {
        const [bridgeRef, section] = ref.split(':');
        const bridgeNumber = parseInt(bridgeRef.replace('bridge-', ''));
        
        if (bridgeNumber >= 1 && bridgeNumber <= 7) {
            this.navigateToBridge(bridgeNumber);
            
            // After navigation, scroll to specific section
            setTimeout(() => {
                const sectionElement = document.querySelector(`[data-section="${section}"]`);
                if (sectionElement) {
                    sectionElement.scrollIntoView({ 
                        behavior: 'smooth', 
                        block: 'center' 
                    });
                    
                    // Highlight the referenced section
                    sectionElement.style.backgroundColor = 'var(--wisconsin-gold)';
                    sectionElement.style.transition = 'background-color 2s ease';
                    
                    setTimeout(() => {
                        sectionElement.style.backgroundColor = '';
                    }, 2000);
                }
            }, 500);
        }
    }
    
    setupSearchFunctionality() {
        const searchInput = document.querySelector('#bridge-search');
        if (searchInput) {
            let searchTimeout;
            
            searchInput.addEventListener('input', (e) => {
                clearTimeout(searchTimeout);
                searchTimeout = setTimeout(() => {
                    this.performSearch(e.target.value);
                }, 300);
            });
        }
    }
    
    performSearch(query) {
        if (query.length < 3) {
            this.clearSearchResults();
            return;
        }
        
        // Mock search implementation
        const results = this.searchContent(query);
        this.displaySearchResults(results);
    }
    
    searchContent(query) {
        // This would interface with a real search engine
        const mockResults = [
            {
                bridge: 1,
                title: 'Recursive Architecture Principles',
                excerpt: `...systems that can modify their own ${query}...`,
                score: 0.95
            },
            {
                bridge: 3,
                title: 'Pattern Recognition in AI',
                excerpt: `...machine learning models that recognize ${query} patterns...`,
                score: 0.87
            }
        ];
        
        return mockResults.filter(result => 
            result.title.toLowerCase().includes(query.toLowerCase()) ||
            result.excerpt.toLowerCase().includes(query.toLowerCase())
        );
    }
    
    displaySearchResults(results) {
        const resultsContainer = document.querySelector('#search-results');
        if (!resultsContainer) return;
        
        resultsContainer.innerHTML = '';
        
        results.forEach(result => {
            const resultElement = document.createElement('div');
            resultElement.className = 'search-result';
            resultElement.innerHTML = `
                <h4><a href="#bridge-${result.bridge}">${result.title}</a></h4>
                <p>${result.excerpt}</p>
                <span class="bridge-indicator">Bridge ${result.bridge}</span>
            `;
            
            resultsContainer.appendChild(resultElement);
        });
    }
    
    clearSearchResults() {
        const resultsContainer = document.querySelector('#search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = '';
        }
    }
    
    setupVerificationMatrix() {
        const verificationItems = document.querySelectorAll('.verification-item');
        
        verificationItems.forEach(item => {
            item.addEventListener('click', () => {
                this.showVerificationDetails(item.getAttribute('data-bot'));
            });
        });
    }
    
    showVerificationDetails(botName) {
        const modal = document.createElement('div');
        modal.className = 'verification-modal';
        modal.innerHTML = `
            <div class="modal-content">
                <h3>${this.getBotDisplayName(botName)} Verification Details</h3>
                <div class="verification-details">
                    ${this.getVerificationDetails(botName)}
                </div>
                <button onclick="this.parentElement.parentElement.remove()">Close</button>
            </div>
        `;
        
        document.body.appendChild(modal);
    }
    
    getBotDisplayName(botName) {
        const botNames = {
            'recursive-architect': 'The Recursive Architect',
            'strategic-tangler': 'The Strategic Tangler',
            'pattern-recognizer': 'The Pattern Recognizer',
            'experience-mapper': 'The Experience Mapper',
            'economic-calculator': 'The Economic Calculator',
            'ethical-recursionist': 'The Ethical Recursionist',
            'vision-synthesizer': 'The Vision Synthesizer'
        };
        
        return botNames[botName] || botName;
    }
    
    getVerificationDetails(botName) {
        // Mock verification details
        return `
            <p><strong>Verification Status:</strong> ✅ Verified</p>
            <p><strong>Last Check:</strong> ${new Date().toLocaleDateString()}</p>
            <p><strong>Confidence Score:</strong> 98.5%</p>
            <p><strong>Issues Found:</strong> 0</p>
            <p><strong>Signature Process:</strong> Strange Loops of Technical Validation</p>
        `;
    }
    
    setupMathRendering() {
        // Load MathJax for LaTeX rendering
        if (window.MathJax) {
            MathJax.typesetPromise();
        } else {
            const script = document.createElement('script');
            script.src = 'https://polyfill.io/v3/polyfill.min.js?features=es6';
            script.onload = () => {
                const mathJaxScript = document.createElement('script');
                mathJaxScript.id = 'MathJax-script';
                mathJaxScript.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js';
                mathJaxScript.onload = () => {
                    window.MathJax = {
                        tex: {
                            inlineMath: [['$', '$'], ['\\(', '\\)']],
                            displayMath: [['$$', '$$'], ['\\[', '\\]']]
                        }
                    };
                };
                document.head.appendChild(mathJaxScript);
            };
            document.head.appendChild(script);
        }
    }
    
    setupReadingProgress() {
        let ticking = false;
        
        const updateProgress = () => {
            const scrolled = window.pageYOffset;
            const totalHeight = document.documentElement.scrollHeight - window.innerHeight;
            const progress = (scrolled / totalHeight) * 100;
            
            const progressBar = document.querySelector('#reading-progress');
            if (progressBar) {
                progressBar.style.width = `${progress}%`;
            }
            
            ticking = false;
        };
        
        window.addEventListener('scroll', () => {
            if (!ticking) {
                requestAnimationFrame(updateProgress);
                ticking = true;
            }
        });
    }
    
    updateReadingProgress() {
        // Update based on current bridge
        const progressIndicator = document.querySelector('#bridge-progress');
        if (progressIndicator) {
            progressIndicator.textContent = `Bridge ${this.currentBridge} of 7`;
        }
    }
}

// Initialize the portal when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new SevenBridgePortal();
});

// Export for potential module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = SevenBridgePortal;
}
