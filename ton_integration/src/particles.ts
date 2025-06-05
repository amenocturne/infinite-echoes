// Particle system
function createParticle(): HTMLDivElement {
  const particle = document.createElement('div');
  particle.className = 'particle';

  // Random size
  const sizes = ['small', 'medium', 'large'];
  const randomSize = sizes[Math.floor(Math.random() * sizes.length)];
  particle.classList.add(randomSize);

  // Random horizontal position (fully random across screen width)
  particle.style.left = Math.random() * 100 + '%';
  particle.style.top = (Math.random() * 100 + 20) + '%';

  // Random animation duration (speed)
  const duration = Math.random() * 15 + 8; // 8-23 seconds
  particle.style.animationDuration = duration + 's';

  // Random delay
  const delay = Math.random() * 3;
  particle.style.animationDelay = '0s';

  // Random horizontal drift during animation
  const drift = (Math.random() - 0.5) * 200; // -100px to 100px drift
  particle.style.setProperty('--drift', drift + 'px');

  return particle;
}

export function initParticles(): void {
  const container = document.getElementById('particles-container');
  if (!container) {
    console.error("Particles container not found!");
    return;
  }

  // Clear any existing particles
  container.innerHTML = '';

  // Create initial particles with staggered delays
  for (let i = 0; i < 100; i++) {
    setTimeout(() => {
      const particle = createParticle();
      container.appendChild(particle);
    }, i * 30);
  }

  // Continuously add new particles
  const particleInterval = setInterval(() => {
    const container = document.getElementById('particles-container');
    if (container && container.parentElement) {
      // Remove old particles to prevent memory buildup
      const particles = container.querySelectorAll('.particle');
      if (particles.length > 200) {
        particles[0].remove();
      }

      // Add new particle
      const particle = createParticle();
      container.appendChild(particle);
    } else {
      // Clear interval if container is removed
      clearInterval(particleInterval);
    }
  }, 50);
}
