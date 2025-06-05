function createParticle(): HTMLDivElement {
  const particle = document.createElement('div');
  particle.className = 'particle';

  const sizes = ['small', 'medium', 'large'];
  const randomSize = sizes[Math.floor(Math.random() * sizes.length)];
  particle.classList.add(randomSize);

  particle.style.left = Math.random() * 100 + '%';
  particle.style.top = (Math.random() * 100 + 20) + '%';

  const duration = Math.random() * 15 + 8;
  particle.style.animationDuration = duration + 's';

  const delay = Math.random() * 3;
  particle.style.animationDelay = '0s';

  const drift = (Math.random() - 0.5) * 200;
  particle.style.setProperty('--drift', drift + 'px');

  return particle;
}

export function initParticles(): void {
  const container = document.getElementById('particles-container');
  if (!container) {
    console.error("Particles container not found!");
    return;
  }

  container.innerHTML = '';

  for (let i = 0; i < 100; i++) {
    setTimeout(() => {
      const particle = createParticle();
      container.appendChild(particle);
    }, i * 30);
  }

  const particleInterval = setInterval(() => {
    const container = document.getElementById('particles-container');
    if (container && container.parentElement) {
      const particles = container.querySelectorAll('.particle');
      if (particles.length > 200) {
        particles[0].remove();
      }

      const particle = createParticle();
      container.appendChild(particle);
    } else {
      clearInterval(particleInterval);
    }
  }, 50);
}
