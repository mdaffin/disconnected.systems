// This is where project configuration and plugin options are located.
// Learn more: https://gridsome.org/docs/config

// Changes here require a server restart.
// To restart press CTRL + C in terminal and run `gridsome develop`

module.exports = {
  siteName: 'Disconnected Systems',
  siteUrl: 'https://disconnected.systems',
  siteDescription: "Programming, Linux and all things Open Source",
  plugins: [
    {
      use: '@gridsome/source-filesystem',
      options: {
        typeName: 'Blog',
        pathPrefix: 'blog',
        path: './content/blog/**/*.md',
      }
    }
  ]
}
