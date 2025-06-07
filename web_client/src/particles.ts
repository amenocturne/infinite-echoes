import { PARTICLE_CONFIG } from './config/constants';

/**
 * Creates a single particle element with randomized properties
 */
function createParticle(): HTMLDivElement {
  const particle = document.createElement('div');
  particle.className = 'particle';

  const randomSize =
    PARTICLE_CONFIG.SIZES[Math.floor(Math.random() * PARTICLE_CONFIG.SIZES.length)];
  particle.classList.add(randomSize);

  particle.style.left = Math.random() * 100 + '%';
  particle.style.top = Math.random() * 100 + 20 + '%';

  const duration =
    Math.random() * (PARTICLE_CONFIG.MAX_DURATION - PARTICLE_CONFIG.MIN_DURATION) +
    PARTICLE_CONFIG.MIN_DURATION;
  particle.style.animationDuration = duration + 's';
  particle.style.animationDelay = '0s';

  const drift = (Math.random() - 0.5) * PARTICLE_CONFIG.MAX_DRIFT;
  particle.style.setProperty('--drift', drift + 'px');

  return particle;
}

/**
 * Initializes the particle system
 */
export function initParticles(): void {
  const container = document.getElementById('particles-container');
  if (!container) {
    console.error('Particles container not found!');
    return;
  }

  container.innerHTML = '';

  for (let i = 0; i < PARTICLE_CONFIG.COUNT; i++) {
    setTimeout(() => {
      const particle = createParticle();
      container.appendChild(particle);
    }, i * 30);
  }

  const particleInterval = setInterval(() => {
    const container = document.getElementById('particles-container');
    if (container && container.parentElement) {
      const particles = container.querySelectorAll('.particle');
      if (particles.length > PARTICLE_CONFIG.MAX_COUNT) {
        particles[0].remove();
      }

      const particle = createParticle();
      container.appendChild(particle);
    } else {
      clearInterval(particleInterval);
    }
  }, PARTICLE_CONFIG.INTERVAL_MS);
}
